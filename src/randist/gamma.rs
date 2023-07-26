//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/// This function computes the probability density p(x) at x for a gamma distribution with parameters a and b, using the formula given above.
#[doc(alias = "gsl_ran_gamma_pdf")]
pub fn gamma_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { sys::gsl_ran_gamma_pdf(x, a, b) }
}

/// This function compute the cumulative distribution functions P(x), Q(x) and their inverses for
/// the gamma distribution with parameters a and b.
#[doc(alias = "gsl_cdf_gamma_P")]
pub fn gamma_P(x: f64, a: f64, b: f64) -> f64 {
    unsafe { sys::gsl_cdf_gamma_P(x, a, b) }
}

/// This function compute the cumulative distribution functions P(x), Q(x) and their inverses for
/// the gamma distribution with parameters a and b.
#[doc(alias = "gsl_cdf_gamma_Q")]
pub fn gamma_Q(x: f64, a: f64, b: f64) -> f64 {
    unsafe { sys::gsl_cdf_gamma_Q(x, a, b) }
}

/// This function compute the cumulative distribution functions P(x), Q(x) and their inverses for
/// the gamma distribution with parameters a and b.
#[doc(alias = "gsl_cdf_gamma_Pinv")]
pub fn gamma_Pinv(P: f64, a: f64, b: f64) -> f64 {
    unsafe { sys::gsl_cdf_gamma_Pinv(P, a, b) }
}

/// This function compute the cumulative distribution functions P(x), Q(x) and their inverses for
/// the gamma distribution with parameters a and b.
#[doc(alias = "gsl_cdf_gamma_Qinv")]
pub fn gamma_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    unsafe { sys::gsl_cdf_gamma_Qinv(Q, a, b) }
}
