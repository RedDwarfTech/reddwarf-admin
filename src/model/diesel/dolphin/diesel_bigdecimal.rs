use bigdecimal::{BigDecimal, num_bigint::{Sign, self}};

use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, ToSql},
    sql_types::Numeric, backend::Backend,
};
use std::io::Write;
use byteorder::{BigEndian, ByteOrder};

impl FromSql<Numeric, Pg> for BigDecimal {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        let mut buf = &bytes[..];
        let (scale, weight, _, neg) = <(i16, i16, i16, u16) as FromSql<Numeric, Pg>>::from_sql(Some(&buf))?;

        let mut digits = vec![0u16; (buf.len() / 2) as usize];
        for digit in &mut digits {
            *digit = buf.read_u16::<BigEndian>()?;
        }

        Ok(bigdecimal::BigDecimal::new(
            num_bigint::BigInt::from_radix_be(if neg == 0 { Sign::Plus } else { Sign::Minus }, &digits, 10000)
                .unwrap(),
            -((scale as i32) - (weight * 4) as i32),
        ))
    }
}

impl ToSql<Numeric, Pg> for BigDecimal {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        let (sign, digits, exponent) = self.as_bigint_and_exponent();
        let scale = -exponent;

        let mut buf = Vec::new();
        for digit in &digits {
            buf.write_u16::<BigEndian>(*digit)?;
        }

        <(i16, i16, i16, u16) as ToSql<Numeric, Pg>>::to_sql(
            &(scale, 0, 0, if sign == num_bigint::Sign::NoSign || sign == num_bigint::Sign::Plus { 0 } else { 1 }),
            out,
        )?;
        out.write_all(&buf)?;

        Ok(serialize::IsNull::No)
    }
}