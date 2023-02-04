use core::marker;

/// Raw register type (`u8`, `u16`, `u32`, ...)
pub trait RawReg:
    Copy
    + Default
    + From<bool>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOrAssign
    + core::ops::BitAndAssign
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u8, Output = Self>
{
    /// Mask for bits of width `WI`
    fn mask<const WI: u8>() -> Self;
    /// Mask for bits of width 1
    fn one() -> Self;
}

macro_rules! raw_reg {
    ($U:ty, $size:literal, $mask:ident) => {
        impl RawReg for $U {
            #[inline(always)]
            fn mask<const WI: u8>() -> Self {
                $mask::<WI>()
            }
            #[inline(always)]
            fn one() -> Self {
                1
            }
        }
        const fn $mask<const WI: u8>() -> $U {
            <$U>::MAX >> ($size - WI)
        }
    };
}

raw_reg!(u8, 8, mask_u8);
raw_reg!(u16, 16, mask_u16);
raw_reg!(u32, 32, mask_u32);
raw_reg!(u64, 64, mask_u64);

/// Raw register type
pub trait RegisterSpec {
    /// Raw register type (`u8`, `u16`, `u32`, ...).
    type Ux: RawReg;
}

/// Trait implemented by readable registers to enable the `read` method.
///
/// Registers marked with `Writable` can be also be `modify`'ed.
pub trait Readable: RegisterSpec {
    /// Result from a call to `read` and argument to `modify`.
    type Reader: From<R<Self>> + core::ops::Deref<Target = R<Self>>;
}

/// Trait implemented by writeable registers.
///
/// This enables the  `write`, `write_with_zero` and `reset` methods.
///
/// Registers marked with `Readable` can be also be `modify`'ed.
pub trait Writable: RegisterSpec {
    /// Writer type argument to `write`, et al.
    type Writer: From<W<Self>> + core::ops::DerefMut<Target = W<Self>>;

    /// Specifies the register bits that are not changed if you pass `1` and are changed if you pass `0`
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux;

    /// Specifies the register bits that are not changed if you pass `0` and are changed if you pass `1`
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux;
}

/// Reset value of the register.
///
/// This value is the initial value for the `write` method. It can also be directly written to the
/// register by using the `reset` method.
pub trait Resettable: RegisterSpec {
    /// Reset value of the register.
    const RESET_VALUE: Self::Ux;

    /// Reset value of the register.
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        Self::RESET_VALUE
    }
}

/// This structure provides volatile access to registers.
#[repr(transparent)]
pub struct Reg<REG: RegisterSpec> {
    register: vcell::VolatileCell<REG::Ux>,
    _marker: marker::PhantomData<REG>,
}

unsafe impl<REG: RegisterSpec> Send for Reg<REG> where REG::Ux: Send {}

impl<REG: RegisterSpec> Reg<REG> {
    /// Returns the underlying memory address of register.
    ///
    /// ```ignore
    /// let reg_ptr = periph.reg.as_ptr();
    /// ```
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut REG::Ux {
        self.register.as_ptr()
    }
}

impl<REG: Readable> Reg<REG> {
    /// Reads the contents of a `Readable` register.
    ///
    /// You can read the raw contents of a register by using `bits`:
    /// ```ignore
    /// let bits = periph.reg.read().bits();
    /// ```
    /// or get the content of a particular field of a register:
    /// ```ignore
    /// let reader = periph.reg.read();
    /// let bits = reader.field1().bits();
    /// let flag = reader.field2().bit_is_set();
    /// ```
    #[inline(always)]
    pub fn read(&self) -> REG::Reader {
        REG::Reader::from(R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        })
    }
}

impl<REG: Resettable + Writable> Reg<REG> {
    /// Writes the reset value to `Writable` register.
    ///
    /// Resets the register to its initial state.
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(REG::RESET_VALUE)
    }

    /// Writes bits to a `Writable` register.
    ///
    /// You can write raw bits into a register:
    /// ```ignore
    /// periph.reg.write(|w| unsafe { w.bits(rawbits) });
    /// ```
    /// or write only the fields you need:
    /// ```ignore
    /// periph.reg.write(|w| w
    ///     .field1().bits(newfield1bits)
    ///     .field2().set_bit()
    ///     .field3().variant(VARIANT)
    /// );
    /// ```
    /// or an alternative way of saying the same:
    /// ```ignore
    /// periph.reg.write(|w| {
    ///     w.field1().bits(newfield1bits);
    ///     w.field2().set_bit();
    ///     w.field3().variant(VARIANT)
    /// });
    /// ```
    /// In the latter case, other fields will be set to their reset value.
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::RESET_VALUE & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                    | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}

impl<REG: Writable> Reg<REG> {
    /// Writes 0 to a `Writable` register.
    ///
    /// Similar to `write`, but unused bits will contain 0.
    ///
    /// # Safety
    ///
    /// Unsafe to use with registers which don't allow to write 0.
    #[inline(always)]
    pub unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::Ux::default(),
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}

impl<REG: Readable + Writable> Reg<REG> {
    /// Modifies the contents of the register by reading and then writing it.
    ///
    /// E.g. to do a read-modify-write sequence to change parts of a register:
    /// ```ignore
    /// periph.reg.modify(|r, w| unsafe { w.bits(
    ///    r.bits() | 3
    /// ) });
    /// ```
    /// or
    /// ```ignore
    /// periph.reg.modify(|_, w| w
    ///     .field1().bits(newfield1bits)
    ///     .field2().set_bit()
    ///     .field3().variant(VARIANT)
    /// );
    /// ```
    /// or an alternative way of saying the same:
    /// ```ignore
    /// periph.reg.modify(|_, w| {
    ///     w.field1().bits(newfield1bits);
    ///     w.field2().set_bit();
    ///     w.field3().variant(VARIANT)
    /// });
    /// ```
    /// Other fields will have the value they had before the call to `modify`.
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&REG::Reader, &'w mut REG::Writer) -> &'w mut W<REG>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &REG::Reader::from(R {
                    bits,
                    _reg: marker::PhantomData,
                }),
                &mut REG::Writer::from(W {
                    bits: bits & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                        | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                    _reg: marker::PhantomData,
                }),
            )
            .bits,
        );
    }
}

/// Register reader.
///
/// Result of the `read` methods of registers. Also used as a closure argument in the `modify`
/// method.
pub struct R<REG: RegisterSpec + ?Sized> {
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}

impl<REG: RegisterSpec> R<REG> {
    /// Reads raw bits from register.
    #[inline(always)]
    pub fn bits(&self) -> REG::Ux {
        self.bits
    }
}

impl<REG: RegisterSpec, FI> PartialEq<FI> for R<REG>
where
    REG::Ux: PartialEq,
    FI: Copy,
    REG::Ux: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&REG::Ux::from(*other))
    }
}

/// Register writer.
///
/// Used as an argument to the closures in the `write` and `modify` methods of the register.
pub struct W<REG: RegisterSpec + ?Sized> {
    ///Writable bits
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}

impl<REG: RegisterSpec> W<REG> {
    /// Writes raw bits to the register.
    ///
    /// # Safety
    ///
    /// Read datasheet or reference manual to find what values are allowed to pass.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: REG::Ux) -> &mut Self {
        self.bits = bits;
        self
    }
}

#[doc(hidden)]
pub struct FieldReaderRaw<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}

impl<U, FI> FieldReaderRaw<U, FI>
where
    U: Copy,
{
    /// Creates a new instance of the reader.
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}

#[doc(hidden)]
pub struct BitReaderRaw<T> {
    pub(crate) bits: bool,
    _reg: marker::PhantomData<T>,
}

impl<FI> BitReaderRaw<FI> {
    /// Creates a new instance of the reader.
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}

/// Field reader.
///
/// Result of the `read` methods of fields.
pub type FieldReader<U, FI> = FieldReaderRaw<U, FI>;

/// Bit-wise field reader
pub type BitReader<FI> = BitReaderRaw<FI>;

impl<U, FI> FieldReader<U, FI>
where
    U: Copy,
{
    /// Reads raw bits from field.
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}

impl<U, FI> PartialEq<FI> for FieldReader<U, FI>
where
    U: PartialEq,
    FI: Copy,
    U: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&U::from(*other))
    }
}

impl<FI> PartialEq<FI> for BitReader<FI>
where
    FI: Copy,
    bool: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&bool::from(*other))
    }
}

impl<FI> BitReader<FI> {
    /// Value of the field as raw bits.
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    /// Returns `true` if the bit is clear (0).
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    /// Returns `true` if the bit is set (1).
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}

#[doc(hidden)]
pub struct Safe;
#[doc(hidden)]
pub struct Unsafe;

#[doc(hidden)]
pub struct FieldWriterRaw<'a, U, REG, N, FI, Safety, const WI: u8, const O: u8>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(N, FI, Safety)>,
}

impl<'a, U, REG, N, FI, Safety, const WI: u8, const O: u8>
    FieldWriterRaw<'a, U, REG, N, FI, Safety, WI, O>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    /// Creates a new instance of the writer
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}

#[doc(hidden)]
pub struct BitWriterRaw<'a, U, REG, FI, M, const O: u8>
where
    REG: Writable + RegisterSpec<Ux = U>,
    bool: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(FI, M)>,
}

impl<'a, U, REG, FI, M, const O: u8> BitWriterRaw<'a, U, REG, FI, M, O>
where
    REG: Writable + RegisterSpec<Ux = U>,
    bool: From<FI>,
{
    /// Creates a new instance of the writer
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}

/// Write field Proxy with unsafe `bits`
pub type FieldWriter<'a, U, REG, N, FI, const WI: u8, const O: u8> =
    FieldWriterRaw<'a, U, REG, N, FI, Unsafe, WI, O>;
/// Write field Proxy with safe `bits`
pub type FieldWriterSafe<'a, U, REG, N, FI, const WI: u8, const O: u8> =
    FieldWriterRaw<'a, U, REG, N, FI, Safe, WI, O>;

impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriter<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    /// Field width
    pub const WIDTH: u8 = WI;
}

impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriterSafe<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    /// Field width
    pub const WIDTH: u8 = WI;
}

macro_rules! bit_proxy {
    ($writer:ident, $mwv:ident) => {
        #[doc(hidden)]
        pub struct $mwv;

        /// Bit-wise write field proxy
        pub type $writer<'a, U, REG, FI, const O: u8> = BitWriterRaw<'a, U, REG, FI, $mwv, O>;

        impl<'a, U, REG, FI, const OF: u8> $writer<'a, U, REG, FI, OF>
        where
            REG: Writable + RegisterSpec<Ux = U>,
            bool: From<FI>,
        {
            /// Field width
            pub const WIDTH: u8 = 1;
        }
    };
}

macro_rules! impl_bit_proxy {
    ($writer:ident) => {
        impl<'a, U, REG, FI, const OF: u8> $writer<'a, U, REG, FI, OF>
        where
            REG: Writable + RegisterSpec<Ux = U>,
            U: RawReg,
            bool: From<FI>,
        {
            /// Writes bit to the field
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut REG::Writer {
                self.w.bits &= !(U::one() << OF);
                self.w.bits |= (U::from(value) & U::one()) << OF;
                self.w
            }
            /// Writes `variant` to the field
            #[inline(always)]
            pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
                self.bit(bool::from(variant))
            }
        }
    };
}

bit_proxy!(BitWriter, BitM);
bit_proxy!(BitWriter1S, Bit1S);
bit_proxy!(BitWriter0C, Bit0C);
bit_proxy!(BitWriter1C, Bit1C);
bit_proxy!(BitWriter0S, Bit0S);
bit_proxy!(BitWriter1T, Bit1T);
bit_proxy!(BitWriter0T, Bit0T);

impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriter<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg + From<N>,
    N: From<FI>,
{
    /// Writes raw bits to the field
    ///
    /// # Safety
    ///
    /// Passing incorrect value can cause undefined behaviour. See reference manual
    #[inline(always)]
    pub unsafe fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(U::mask::<WI>() << OF);
        self.w.bits |= (U::from(value) & U::mask::<WI>()) << OF;
        self.w
    }
    /// Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        unsafe { self.bits(N::from(variant)) }
    }
}
impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriterSafe<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg + From<N>,
    N: From<FI>,
{
    /// Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(U::mask::<WI>() << OF);
        self.w.bits |= (U::from(value) & U::mask::<WI>()) << OF;
        self.w
    }
    /// Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        self.bits(N::from(variant))
    }
}

impl_bit_proxy!(BitWriter);
impl_bit_proxy!(BitWriter1S);
impl_bit_proxy!(BitWriter0C);
impl_bit_proxy!(BitWriter1C);
impl_bit_proxy!(BitWriter0S);
impl_bit_proxy!(BitWriter1T);
impl_bit_proxy!(BitWriter0T);

impl<'a, U, REG, FI, const OF: u8> BitWriter<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    /// Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
    /// Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter1S<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    /// Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter0C<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    /// Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter1C<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    ///Clears the field bit by passing one
    #[inline(always)]
    pub fn clear_bit_by_one(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter0S<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    ///Sets the field bit by passing zero
    #[inline(always)]
    pub fn set_bit_by_zero(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter1T<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    ///Toggle the field bit by passing one
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}

impl<'a, U, REG, FI, const OF: u8> BitWriter0T<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    ///Toggle the field bit by passing zero
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}

mod atomic {
    use portable_atomic::Ordering;

    pub trait AtomicOperations {
        unsafe fn atomic_or(ptr: *mut Self, val: Self);
        unsafe fn atomic_and(ptr: *mut Self, val: Self);
        unsafe fn atomic_xor(ptr: *mut Self, val: Self);
    }

    macro_rules! impl_atomics {
        ($U:ty, $Atomic:ty) => {
            impl AtomicOperations for $U {
                unsafe fn atomic_or(ptr: *mut Self, val: Self) {
                    (*(ptr as *const $Atomic)).or(val, Ordering::SeqCst);
                }

                unsafe fn atomic_and(ptr: *mut Self, val: Self) {
                    (*(ptr as *const $Atomic)).and(val, Ordering::SeqCst);
                }

                unsafe fn atomic_xor(ptr: *mut Self, val: Self) {
                    (*(ptr as *const $Atomic)).xor(val, Ordering::SeqCst);
                }
            }
        };
    }

    impl_atomics!(u8, portable_atomic::AtomicU8);
    impl_atomics!(u16, portable_atomic::AtomicU16);

    // Exclude 16-bit archs from 32-bit atomics
    #[cfg(not(target_pointer_width = "16"))]
    impl_atomics!(u32, portable_atomic::AtomicU32);

    // Enable 64-bit atomics for 64-bit RISCV
    #[cfg(any(target_pointer_width = "64", target_has_atomic = "64"))]
    impl_atomics!(u64, portable_atomic::AtomicU64);
}
use atomic::AtomicOperations;

impl<REG: Readable + Writable> Reg<REG>
where
    REG::Ux: AtomicOperations + Default + core::ops::Not<Output = REG::Ux>,
{
    /// Set high every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    ///
    /// # Safety
    ///
    /// The resultant bit pattern may not be valid for the register.
    #[inline(always)]
    pub unsafe fn set_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        REG::Ux::atomic_or(self.register.as_ptr(), bits);
    }

    /// Clear every bit in the register that was cleared in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    ///
    /// # Safety
    ///
    /// The resultant bit pattern may not be valid for the register.
    #[inline(always)]
    pub unsafe fn clear_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: !REG::Ux::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        REG::Ux::atomic_and(self.register.as_ptr(), bits);
    }

    /// Toggle every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    ///
    /// # Safety
    ///
    /// The resultant bit pattern may not be valid for the register.
    #[inline(always)]
    pub unsafe fn toggle_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        REG::Ux::atomic_xor(self.register.as_ptr(), bits);
    }
}
