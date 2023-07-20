//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

#[derive(Clone, Copy)]
pub struct Index(pub(crate) sys::CBLAS_INDEX_t);

impl Index {
    pub fn new(value: usize) -> Index {
        Index(value)
    }
}

pub mod level1 {
    #[doc(alias = "cblas_sdsdot")]
    pub fn sdsdot(N: i32, alpha: f32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
        unsafe { ::sys::cblas_sdsdot(N, alpha, x.as_ptr(), incx, y.as_ptr(), incy) }
    }

    #[doc(alias = "cblas_dsdot")]
    pub fn dsdot(N: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f64 {
        unsafe { ::sys::cblas_dsdot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
    }

    #[doc(alias = "cblas_sdot")]
    pub fn sdot(N: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
        unsafe { ::sys::cblas_sdot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
    }

    #[doc(alias = "cblas_ddot")]
    pub fn ddot(N: i32, x: &[f64], incx: i32, y: &[f64], incy: i32) -> f64 {
        unsafe { ::sys::cblas_ddot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
    }

    #[doc(alias = "cblas_cdotu_sub")]
    pub fn cdotu_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotu: &mut [T]) {
        unsafe {
            ::sys::cblas_cdotu_sub(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                dotu.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_cdotc_sub")]
    pub fn cdotc_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotc: &mut [T]) {
        unsafe {
            ::sys::cblas_cdotc_sub(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                dotc.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_zdotu_sub")]
    pub fn zdotu_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotu: &mut [T]) {
        unsafe {
            ::sys::cblas_zdotu_sub(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                dotu.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_zdotc_sub")]
    pub fn zdotc_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotc: &mut [T]) {
        unsafe {
            ::sys::cblas_zdotc_sub(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                dotc.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_snrm2")]
    pub fn snrm2(N: i32, x: &[f32], incx: i32) -> f32 {
        unsafe { ::sys::cblas_snrm2(N, x.as_ptr(), incx) }
    }

    #[doc(alias = "cblas_sasum")]
    pub fn sasum(N: i32, x: &[f32], incx: i32) -> f32 {
        unsafe { ::sys::cblas_sasum(N, x.as_ptr(), incx) }
    }

    #[doc(alias = "cblas_dnrm2")]
    pub fn dnrm2(N: i32, x: &[f64], incx: i32) -> f64 {
        unsafe { ::sys::cblas_dnrm2(N, x.as_ptr(), incx) }
    }

    #[doc(alias = "cblas_dasum")]
    pub fn dasum(N: i32, x: &[f64], incx: i32) -> f64 {
        unsafe { ::sys::cblas_dasum(N, x.as_ptr(), incx) }
    }

    #[doc(alias = "cblas_scnrm2")]
    pub fn scnrm2<T>(N: i32, x: &[T], incx: i32) -> f32 {
        unsafe { ::sys::cblas_scnrm2(N, x.as_ptr() as *const _, incx) }
    }

    #[doc(alias = "cblas_scasum")]
    pub fn scasum<T>(N: i32, x: &[T], incx: i32) -> f32 {
        unsafe { ::sys::cblas_scasum(N, x.as_ptr() as *const _, incx) }
    }

    #[doc(alias = "cblas_dznrm2")]
    pub fn dznrm2<T>(N: i32, x: &[T], incx: i32) -> f64 {
        unsafe { ::sys::cblas_dznrm2(N, x.as_ptr() as *const _, incx) }
    }

    #[doc(alias = "cblas_dzasum")]
    pub fn dzasum<T>(N: i32, x: &[T], incx: i32) -> f64 {
        unsafe { ::sys::cblas_dzasum(N, x.as_ptr() as *const _, incx) }
    }

    #[doc(alias = "cblas_isamax")]
    pub fn isamax(N: i32, x: &[f32], incx: i32) -> ::cblas::Index {
        ::cblas::Index(unsafe { ::sys::cblas_isamax(N, x.as_ptr(), incx) })
    }

    #[doc(alias = "cblas_idamax")]
    pub fn idamax(N: i32, x: &[f64], incx: i32) -> ::cblas::Index {
        ::cblas::Index(unsafe { ::sys::cblas_idamax(N, x.as_ptr(), incx) })
    }

    #[doc(alias = "cblas_icamax")]
    pub fn icamax<T>(N: i32, x: &[T], incx: i32) -> ::cblas::Index {
        ::cblas::Index(unsafe { ::sys::cblas_icamax(N, x.as_ptr() as *const _, incx) })
    }

    #[doc(alias = "cblas_izamax")]
    pub fn izamax<T>(N: i32, x: &[T], incx: i32) -> ::cblas::Index {
        ::cblas::Index(unsafe { ::sys::cblas_izamax(N, x.as_ptr() as *const _, incx) })
    }

    #[doc(alias = "cblas_sswap")]
    pub fn sswap(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32) {
        unsafe { ::sys::cblas_sswap(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_scopy")]
    pub fn scopy(N: i32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
        unsafe { ::sys::cblas_scopy(N, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_saxpy")]
    pub fn saxpy(N: i32, alpha: f32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
        unsafe { ::sys::cblas_saxpy(N, alpha, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_dswap")]
    pub fn dswap(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32) {
        unsafe { ::sys::cblas_dswap(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_dcopy")]
    pub fn dcopy(N: i32, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {
        unsafe { ::sys::cblas_dcopy(N, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_daxpy")]
    pub fn daxpy(N: i32, alpha: f64, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {
        unsafe { ::sys::cblas_daxpy(N, alpha, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
    }

    #[doc(alias = "cblas_cswap")]
    pub fn cswap<T>(N: i32, x: &mut [T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_cswap(
                N,
                x.as_mut_ptr() as *mut _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_ccopy")]
    pub fn ccopy<T>(N: i32, x: &[T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_ccopy(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_caxpy")]
    pub fn caxpy<T>(N: i32, alpha: &[T], x: &[T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_caxpy(
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zswap")]
    pub fn zswap<T>(N: i32, x: &mut [T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_zswap(
                N,
                x.as_mut_ptr() as *mut _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zcopy")]
    pub fn zcopy<T>(N: i32, x: &[T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_zcopy(
                N,
                x.as_ptr() as *const _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zaxpy")]
    pub fn zaxpy<T>(N: i32, alpha: &[T], x: &[T], incx: i32, y: &mut [T], incy: i32) {
        unsafe {
            ::sys::cblas_zaxpy(
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_srotg")]
    pub fn srotg(a: &mut [f32], b: &mut [f32], c: &mut [f32], s: &mut [f32]) {
        unsafe {
            ::sys::cblas_srotg(
                a.as_mut_ptr(),
                b.as_mut_ptr(),
                c.as_mut_ptr(),
                s.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_srotmg")]
    pub fn srotmg(d1: &mut [f32], d2: &mut [f32], b1: &mut [f32], b2: f32, P: &mut [f32]) {
        unsafe {
            ::sys::cblas_srotmg(
                d1.as_mut_ptr(),
                d2.as_mut_ptr(),
                b1.as_mut_ptr(),
                b2,
                P.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_srot")]
    pub fn srot(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32, c: f32, s: f32) {
        unsafe { ::sys::cblas_srot(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, c, s) }
    }

    #[doc(alias = "cblas_srotm")]
    pub fn srotm(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32, p: &[f32]) {
        unsafe { ::sys::cblas_srotm(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, p.as_ptr()) }
    }

    #[doc(alias = "cblas_drotg")]
    pub fn drotg(a: &mut [f64], b: &mut [f64], c: &mut [f64], s: &mut [f64]) {
        unsafe {
            ::sys::cblas_drotg(
                a.as_mut_ptr(),
                b.as_mut_ptr(),
                c.as_mut_ptr(),
                s.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_drotmg")]
    pub fn drotmg(d1: &mut [f64], d2: &mut [f64], b1: &mut [f64], b2: f64, P: &mut [f64]) {
        unsafe {
            ::sys::cblas_drotmg(
                d1.as_mut_ptr(),
                d2.as_mut_ptr(),
                b1.as_mut_ptr(),
                b2,
                P.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_drot")]
    pub fn drot(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32, c: f64, s: f64) {
        unsafe { ::sys::cblas_drot(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, c, s) }
    }

    #[doc(alias = "cblas_drotm")]
    pub fn drotm(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32, p: &[f64]) {
        unsafe { ::sys::cblas_drotm(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, p.as_ptr()) }
    }

    /// Multiple each element of a matrix/vector by a constant.
    ///
    /// __Postcondition__: Every incX'th element of X has been multiplied by a factor of alpha
    ///
    /// __Parameters__:
    ///
    /// * N : number of elements in x to scale
    /// * alpha : factor to scale by
    /// * X : pointer to the vector/matrix data
    /// * incx : Amount to increment counter after each scaling, ie incX=2 mean to scale elements {1,3,...}
    ///
    /// Note that the allocated length of X must be incX*N-1 as N indicates the number of scaling operations to perform.
    #[doc(alias = "cblas_sscal")]
    pub fn sscal(N: i32, alpha: f32, x: &mut [f32], incx: i32) {
        unsafe { ::sys::cblas_sscal(N, alpha, x.as_mut_ptr(), incx) }
    }

    /// Multiple each element of a matrix/vector by a constant.
    #[doc(alias = "cblas_dscal")]
    pub fn dscal(N: i32, alpha: f64, x: &mut [f64], incx: i32) {
        unsafe { ::sys::cblas_dscal(N, alpha, x.as_mut_ptr(), incx) }
    }

    /// Multiple each element of a matrix/vector by a constant.
    #[doc(alias = "cblas_cscal")]
    pub fn cscal<T>(N: i32, alpha: &[T], x: &mut [T], incx: i32) {
        unsafe {
            ::sys::cblas_cscal(
                N,
                alpha.as_ptr() as *const _,
                x.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    /// Multiple each element of a matrix/vector by a constant.
    #[doc(alias = "cblas_zscal")]
    pub fn zscal<T>(N: i32, alpha: &[T], x: &mut [T], incx: i32) {
        unsafe {
            ::sys::cblas_zscal(
                N,
                alpha.as_ptr() as *const _,
                x.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    /// Multiple each element of a matrix/vector by a constant.
    #[doc(alias = "cblas_csscal")]
    pub fn csscal<T>(N: i32, alpha: f32, x: &mut [T], incx: i32) {
        unsafe { ::sys::cblas_csscal(N, alpha, x.as_mut_ptr() as *mut _, incx) }
    }

    /// Multiple each element of a matrix/vector by a constant.
    #[doc(alias = "cblas_zdscal")]
    pub fn zdscal<T>(N: i32, alpha: f64, x: &mut [T], incx: i32) {
        unsafe { ::sys::cblas_zdscal(N, alpha, x.as_mut_ptr() as *mut _, incx) }
    }
}

pub mod level2 {
    use crate::enums;

    /// Multiplies a matrix and a vector.
    ///
    /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor
    /// * transA :  Whether to transpose matrix A. One of enum CblasNoTrans, CBlasTrans.
    /// * M : Rows in matrix A
    /// * N : Columns in matrix A
    /// * alpha : scalar factor for (sigma * op(A) * x)
    /// * A : matrix A
    /// * lda : The size of the first dimension of matrix A
    /// * X : vector X
    /// * incx : use every incX'th element of X
    /// * beta : scalar factor y
    /// * Y : vector Y
    /// * incy : use every incY'th element of Y
    ///
    /// For parameter lda, if you are passing a matrix `A[m][n]`, the value of parameter lda should be m.
    #[doc(alias = "cblas_sgemv")]
    pub fn sgemv(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        X: &[f32],
        incx: i32,
        beta: f32,
        Y: &mut [f32],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_sgemv(
                order.into(),
                transA.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                X.as_ptr(),
                incx,
                beta,
                Y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_sgbmv")]
    pub fn sgbmv(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        KL: i32,
        KU: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        X: &[f32],
        incx: i32,
        beta: f32,
        Y: &mut [f32],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_sgbmv(
                order.into(),
                transA.into(),
                M,
                N,
                KL,
                KU,
                alpha,
                A.as_ptr(),
                lda,
                X.as_ptr(),
                incx,
                beta,
                Y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_strmv")]
    pub fn strmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[f32],
        lda: i32,
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_strmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_stbmv")]
    pub fn stbmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[f32],
        lda: i32,
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_stbmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_stpmv")]
    pub fn stpmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[f32],
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_stpmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr(),
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_strsv")]
    pub fn strsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[f32],
        lda: i32,
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_strsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_stbsv")]
    pub fn stbsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[f32],
        lda: i32,
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_stbsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_stpsv")]
    pub fn stpsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[f32],
        X: &mut [f32],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_stpsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr(),
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dgemv")]
    pub fn dgemv(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        X: &[f64],
        incx: i32,
        beta: f64,
        Y: &mut [f64],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_dgemv(
                order.into(),
                transA.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                X.as_ptr(),
                incx,
                beta,
                Y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_dgbmv")]
    pub fn dgbmv(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        KL: i32,
        KU: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        X: &[f64],
        incx: i32,
        beta: f64,
        Y: &mut [f64],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_dgbmv(
                order.into(),
                transA.into(),
                M,
                N,
                KL,
                KU,
                alpha,
                A.as_ptr(),
                lda,
                X.as_ptr(),
                incx,
                beta,
                Y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_dtrmv")]
    pub fn dtrmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[f64],
        lda: i32,
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtrmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dtbmv")]
    pub fn dtbmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[f64],
        lda: i32,
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtbmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dtpmv")]
    pub fn dtpmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[f64],
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtpmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr(),
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dtrsv")]
    pub fn dtrsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[f64],
        lda: i32,
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtrsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dtbsv")]
    pub fn dtbsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[f64],
        lda: i32,
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtbsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr(),
                lda,
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_dtpsv")]
    pub fn dtpsv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[f64],
        X: &mut [f64],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_dtpsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr(),
                X.as_mut_ptr(),
                incx,
            )
        }
    }

    #[doc(alias = "cblas_cgemv")]
    pub fn cgemv<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        X: &[T],
        incx: i32,
        beta: &[T],
        Y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_cgemv(
                order.into(),
                transA.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                X.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                Y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_cgbmv")]
    pub fn cgbmv<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        KL: i32,
        KU: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        X: &[T],
        incx: i32,
        beta: &[T],
        Y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_cgbmv(
                order.into(),
                transA.into(),
                M,
                N,
                KL,
                KU,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                X.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                Y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_ctrmv")]
    pub fn ctrmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctrmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ctbmv")]
    pub fn ctbmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctbmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ctpmv")]
    pub fn ctpmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[T],
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctpmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr() as *const _,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ctrsv")]
    pub fn ctrsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctrsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ctbsv")]
    pub fn ctbsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctbsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ctpsv")]
    pub fn ctpsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[T],
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ctpsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr() as *const _,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_zgemv")]
    pub fn zgemv<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        X: &[T],
        incx: i32,
        beta: &[T],
        Y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_zgemv(
                order.into(),
                transA.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                X.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                Y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zgbmv")]
    pub fn zgbmv<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        M: i32,
        N: i32,
        KL: i32,
        KU: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        X: &[T],
        incx: i32,
        beta: &[T],
        Y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_zgbmv(
                order.into(),
                transA.into(),
                M,
                N,
                KL,
                KU,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                X.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                Y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_ztrmv")]
    pub fn ztrmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztrmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ztbmv")]
    pub fn ztbmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztbmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ztpmv")]
    pub fn ztpmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[T],
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztpmv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr() as *const _,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ztrsv")]
    pub fn ztrsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztrsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ztbsv")]
    pub fn ztbsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        K: i32,
        A: &[T],
        lda: i32,
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztbsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                K,
                A.as_ptr() as *const _,
                lda,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ztpsv")]
    pub fn ztpsv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        N: i32,
        Ap: &[T],
        X: &mut [T],
        incx: i32,
    ) {
        unsafe {
            ::sys::cblas_ztpsv(
                order.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                N,
                Ap.as_ptr() as *const _,
                X.as_mut_ptr() as *mut _,
                incx,
            )
        }
    }

    #[doc(alias = "cblas_ssymv")]
    pub fn ssymv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        x: &[f32],
        incx: i32,
        beta: f32,
        y: &mut [f32],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_ssymv(
                order.into(),
                uplo.into(),
                N,
                alpha,
                A.as_ptr(),
                lda,
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_ssbmv")]
    pub fn ssbmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        K: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        x: &[f32],
        incx: i32,
        beta: f32,
        y: &mut [f32],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_ssbmv(
                order.into(),
                uplo.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_sspmv")]
    pub fn sspmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        Ap: &[f32],
        x: &[f32],
        incx: i32,
        beta: f32,
        y: &mut [f32],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_sspmv(
                order.into(),
                uplo.into(),
                N,
                alpha,
                Ap.as_ptr(),
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_sger")]
    pub fn sger(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: f32,
        x: &[f32],
        incx: i32,
        y: &[f32],
        incy: i32,
        A: &mut [f32],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_sger(
                order.into(),
                M,
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_ssyr")]
    pub fn ssyr(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[f32],
        incx: i32,
        A: &mut [f32],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_ssyr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_sspr")]
    pub fn sspr(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[f32],
        incx: i32,
        Ap: &mut [f32],
    ) {
        unsafe {
            ::sys::cblas_sspr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                Ap.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_ssyr2")]
    pub fn ssyr2(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[f32],
        incx: i32,
        y: &[f32],
        incy: i32,
        A: &mut [f32],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_ssyr2(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_sspr2")]
    pub fn sspr2(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[f32],
        incx: i32,
        y: &[f32],
        incy: i32,
        A: &mut [f32],
    ) {
        unsafe {
            ::sys::cblas_sspr2(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_dsymv")]
    pub fn dsymv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        x: &[f64],
        incx: i32,
        beta: f64,
        y: &mut [f64],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_dsymv(
                order.into(),
                uplo.into(),
                N,
                alpha,
                A.as_ptr(),
                lda,
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_dsbmv")]
    pub fn dsbmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        K: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        x: &[f64],
        incx: i32,
        beta: f64,
        y: &mut [f64],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_dsbmv(
                order.into(),
                uplo.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_dspmv")]
    pub fn dspmv(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        Ap: &[f64],
        x: &[f64],
        incx: i32,
        beta: f64,
        y: &mut [f64],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_dspmv(
                order.into(),
                uplo.into(),
                N,
                alpha,
                Ap.as_ptr(),
                x.as_ptr(),
                incx,
                beta,
                y.as_mut_ptr(),
                incy,
            )
        }
    }

    #[doc(alias = "cblas_dger")]
    pub fn dger(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: f64,
        x: &[f64],
        incx: i32,
        y: &[f64],
        incy: i32,
        A: &mut [f64],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_dger(
                order.into(),
                M,
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_dsyr")]
    pub fn dsyr(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[f64],
        incx: i32,
        A: &mut [f64],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_dsyr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_dspr")]
    pub fn dspr(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[f64],
        incx: i32,
        Ap: &mut [f64],
    ) {
        unsafe {
            ::sys::cblas_dspr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                Ap.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_dsyr2")]
    pub fn dsyr2(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[f64],
        incx: i32,
        y: &[f64],
        incy: i32,
        A: &mut [f64],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_dsyr2(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
                lda,
            )
        }
    }

    #[doc(alias = "cblas_dspr2")]
    pub fn dspr2(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[f64],
        incx: i32,
        y: &[f64],
        incy: i32,
        A: &mut [f64],
    ) {
        unsafe {
            ::sys::cblas_dspr2(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr(),
                incx,
                y.as_ptr(),
                incy,
                A.as_mut_ptr(),
            )
        }
    }

    #[doc(alias = "cblas_chemv")]
    pub fn chemv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_chemv(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_chbmv")]
    pub fn chbmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_chbmv(
                order.into(),
                uplo.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_chpmv")]
    pub fn chpmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        Ap: &[T],
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_chpmv(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                Ap.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_cgeru")]
    pub fn cgeru<T>(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_cgeru(
                order.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_cgerc")]
    pub fn cgerc<T>(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_cgerc(
                order.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_cher")]
    pub fn cher<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[T],
        incx: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_cher(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr() as *const _,
                incx,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_chpr")]
    pub fn chpr<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f32,
        x: &[T],
        incx: i32,
        Ap: &mut [T],
    ) {
        unsafe {
            ::sys::cblas_chpr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr() as *const _,
                incx,
                Ap.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_cher2")]
    pub fn cher2<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_cher2(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_chpr2")]
    pub fn chpr2<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[f64],
        incy: i32,
        Ap: &mut [f64],
    ) {
        unsafe {
            ::sys::cblas_chpr2(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                Ap.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_zhemv")]
    pub fn zhemv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_zhemv(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zhbmv")]
    pub fn zhbmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_zhbmv(
                order.into(),
                uplo.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zhpmv")]
    pub fn zhpmv<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        Ap: &[T],
        x: &[T],
        incx: i32,
        beta: &[T],
        y: &mut [T],
        incy: i32,
    ) {
        unsafe {
            ::sys::cblas_zhpmv(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                Ap.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                beta.as_ptr() as *const _,
                y.as_mut_ptr() as *mut _,
                incy,
            )
        }
    }

    #[doc(alias = "cblas_zgeru")]
    pub fn zgeru<T>(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_zgeru(
                order.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_zgerc")]
    pub fn zgerc<T>(
        order: enums::CblasOrder,
        M: i32,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_zgerc(
                order.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_zher")]
    pub fn zher<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[T],
        incx: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_zher(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr() as *const _,
                incx,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_zhpr")]
    pub fn zhpr<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: f64,
        x: &[T],
        incx: i32,
        Ap: &mut [T],
    ) {
        unsafe {
            ::sys::cblas_zhpr(
                order.into(),
                uplo.into(),
                N,
                alpha,
                x.as_ptr() as *const _,
                incx,
                Ap.as_mut_ptr() as *mut _,
            )
        }
    }

    #[doc(alias = "cblas_zher2")]
    pub fn zher2<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[T],
        incy: i32,
        A: &mut [T],
        lda: i32,
    ) {
        unsafe {
            ::sys::cblas_zher2(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                A.as_mut_ptr() as *mut _,
                lda,
            )
        }
    }

    #[doc(alias = "cblas_zhpr2")]
    pub fn zhpr2<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        N: i32,
        alpha: &[T],
        x: &[T],
        incx: i32,
        y: &[f64],
        incy: i32,
        Ap: &mut [f64],
    ) {
        unsafe {
            ::sys::cblas_zhpr2(
                order.into(),
                uplo.into(),
                N,
                alpha.as_ptr() as *const _,
                x.as_ptr() as *const _,
                incx,
                y.as_ptr() as *const _,
                incy,
                Ap.as_mut_ptr() as *mut _,
            )
        }
    }
}

pub mod level3 {
    use crate::enums;

    /// General ::types::Matrix-MatrixF64 multiplication for single precision float.
    ///
    /// __Parameters:__
    ///
    /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor.
    /// * transA : Whether to transpose matrix A. One of enum CblasNoTrans, CBlasTrans, CBlasConjTrans.
    /// * transB : Whether to transpose matrix B. One of enum CblasNoTrans, CBlasTrans, CBlasConjTrans.
    /// * M : Rows in matrices A and C
    /// * N : Columns in Matrices B and C
    /// * K : Columns in matrix A and Rows in matrix B
    /// * alpha : scalar factor for op(A)op(B)
    /// * A : matrix A
    /// * lda : The size of the first dimension of matrix A
    /// * B : matrix B
    /// * ldb : The size of the first dimension of matrix B
    /// * beta : scalar factor for C
    /// * C : matrix C
    /// * ldc : The size of the first dimension of matrix C
    ///
    /// For parameters lda, ldb, and ldc, if you are passing a matrix `D[m][n]`, the value of parameter lda, ldb, or ldc should be m.
    #[doc(alias = "cblas_sgemm")]
    pub fn sgemm(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        transB: enums::CblasTranspose,
        M: i32,
        N: i32,
        K: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        B: &[f32],
        ldb: i32,
        beta: f32,
        C: &mut [f32],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_sgemm(
                order.into(),
                transA.into(),
                transB.into(),
                M,
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    /// Symmetric ::types::Matrix-MatrixF64 multiplication for single precision float.
    ///
    /// __Parameters:__
    ///
    /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor.
    /// * side : If CBlasSideLeft, perform (sigma(A)(B) + beta C). If CBlasSideRight, perform (sigma (B)(A) + beta C)
    /// * uplo : Indicates whether to use the upper (CBlasUpper) or lower (CBlasLower) triangle of matrix A
    /// * M : Rows in matrices A and C
    /// * N : Columns in Matrices B and C
    /// * alpha : scalar factor for op(A)op(B)
    /// * A : matrix A
    /// * lda : The size of the first dimension of matrix A
    /// * B : matrix B
    /// * ldb : The size of the first dimension of matrix B
    /// * beta : scalar factor for C
    /// * C : matrix C
    /// * ldc : The size of the first dimension of matrix C
    #[doc(alias = "cblas_ssymm")]
    pub fn ssymm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        B: &[f32],
        ldb: i32,
        beta: f32,
        C: &mut [f32],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_ssymm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_ssyrk")]
    pub fn ssyrk(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        beta: f32,
        C: &mut [f32],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_ssyrk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_ssyr2k")]
    pub fn ssyr2k(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        B: &[f32],
        ldb: i32,
        beta: f32,
        C: &mut [f32],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_ssyr2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_strmm")]
    pub fn strmm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        B: &mut [f32],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_strmm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_mut_ptr(),
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_strsm")]
    pub fn strsm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: f32,
        A: &[f32],
        lda: i32,
        B: &mut [f32],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_strsm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_mut_ptr(),
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_dgemm")]
    pub fn dgemm(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        transB: enums::CblasTranspose,
        M: i32,
        N: i32,
        K: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        B: &[f64],
        ldb: i32,
        beta: f64,
        C: &mut [f64],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_dgemm(
                order.into(),
                transA.into(),
                transB.into(),
                M,
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_dsymm")]
    pub fn dsymm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        B: &[f64],
        ldb: i32,
        beta: f64,
        C: &mut [f64],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_dsymm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_dsyrk")]
    pub fn dsyrk(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        beta: f64,
        C: &mut [f64],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_dsyrk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_dsyr2k")]
    pub fn dsyr2k(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        B: &[f64],
        ldb: i32,
        beta: f64,
        C: &mut [f64],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_dsyr2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr(),
                lda,
                B.as_ptr(),
                ldb,
                beta,
                C.as_mut_ptr(),
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_dtrmm")]
    pub fn dtrmm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        B: &mut [f64],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_dtrmm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_mut_ptr(),
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_dtrsm")]
    pub fn dtrsm(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: f64,
        A: &[f64],
        lda: i32,
        B: &mut [f64],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_dtrsm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha,
                A.as_ptr(),
                lda,
                B.as_mut_ptr(),
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_cgemm")]
    pub fn cgemm<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        transB: enums::CblasTranspose,
        M: i32,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_cgemm(
                order.into(),
                transA.into(),
                transB.into(),
                M,
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_csymm")]
    pub fn csymm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_csymm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_csyrk")]
    pub fn csyrk<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_csyrk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_csyr2k")]
    pub fn csyr2k<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_csyr2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_ctrmm")]
    pub fn ctrmm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &mut [T],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_ctrmm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_mut_ptr() as *mut _,
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_ctrsm")]
    pub fn ctrsm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &mut [T],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_ctrsm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_mut_ptr() as *mut _,
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_zgemm")]
    pub fn zgemm<T>(
        order: enums::CblasOrder,
        transA: enums::CblasTranspose,
        transB: enums::CblasTranspose,
        M: i32,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zgemm(
                order.into(),
                transA.into(),
                transB.into(),
                M,
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zsymm")]
    pub fn zsymm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zsymm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zsyrk")]
    pub fn zsyrk<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zsyrk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zsyr2k")]
    pub fn zsyr2k<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zsyr2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_ztrmm")]
    pub fn ztrmm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &mut [T],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_ztrmm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_mut_ptr() as *mut _,
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_ztrsm")]
    pub fn ztrsm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        transA: enums::CblasTranspose,
        diag: enums::CblasDiag,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &mut [T],
        ldb: i32,
    ) {
        unsafe {
            ::sys::cblas_ztrsm(
                order.into(),
                side.into(),
                uplo.into(),
                transA.into(),
                diag.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_mut_ptr() as *mut _,
                ldb,
            )
        }
    }

    #[doc(alias = "cblas_chemm")]
    pub fn chemm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_chemm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_cherk")]
    pub fn cherk<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f32,
        A: &[T],
        lda: i32,
        beta: f32,
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_cherk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr() as *const _,
                lda,
                beta,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_cher2k")]
    pub fn cher2k<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: f32,
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_cher2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zhemm")]
    pub fn zhemm<T>(
        order: enums::CblasOrder,
        side: enums::CblasSide,
        uplo: enums::CblasUplo,
        M: i32,
        N: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: &[T],
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zhemm(
                order.into(),
                side.into(),
                uplo.into(),
                M,
                N,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta.as_ptr() as *const _,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zherk")]
    pub fn zherk<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: f64,
        A: &[T],
        lda: i32,
        beta: f64,
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zherk(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha,
                A.as_ptr() as *const _,
                lda,
                beta,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }

    #[doc(alias = "cblas_zher2k")]
    pub fn zher2k<T>(
        order: enums::CblasOrder,
        uplo: enums::CblasUplo,
        trans: enums::CblasTranspose,
        N: i32,
        K: i32,
        alpha: &[T],
        A: &[T],
        lda: i32,
        B: &[T],
        ldb: i32,
        beta: f64,
        C: &mut [T],
        ldc: i32,
    ) {
        unsafe {
            ::sys::cblas_zher2k(
                order.into(),
                uplo.into(),
                trans.into(),
                N,
                K,
                alpha.as_ptr() as *const _,
                A.as_ptr() as *const _,
                lda,
                B.as_ptr() as *const _,
                ldb,
                beta,
                C.as_mut_ptr() as *mut _,
                ldc,
            )
        }
    }
}
