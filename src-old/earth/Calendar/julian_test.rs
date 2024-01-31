#[cfg(test)]
mod tests {
    use crate::shared::datetime::*;

    #[test]
    fn does_since_j200() {
        days_since_j200(1969, 1, 5, 25.083333333333333);
    }

    #[test]
    fn does_jd2greg() {
        let jd = get_jd(1969, 1, 5, 25.083333333333333);
        jd2greg(jd);
    }

    #[test]
    fn does_get_jd() {
        let jd = get_jd(1969, 1, 5, 25.083333333333333);
        println!("{jd}");
    }

    #[test]
    fn jd_today() {
        let jd = get_jd(2024, 1, 15, 12.05);
        jd2greg(jd);
    }
}
