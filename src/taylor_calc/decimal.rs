mod taylor_calc;

extern crate gmp;

mod decimal {
    pub struct Decimal {
        gmpMpq: mpq_struct;
    };
}
