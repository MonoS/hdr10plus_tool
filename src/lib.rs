mod hdr10plus;

//Regression tests
#[cfg(test)]
mod tests {

    use crate::hdr10plus::parser::*;

    // x265 Tool_Verification_new_hdr10plus_llc.json 1st frame
    #[test]
    fn sample1() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s1.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 400);
        assert_eq!(test[0].average_maxrgb, 1037);
        assert_eq!(test[0].maxscl, vec![17830, 16895, 14252]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![3, 14024, 43, 56, 219, 1036, 2714, 4668, 14445]
        );
        assert_eq!(test[0].knee_x, 17);
        assert_eq!(test[0].knee_y, 64);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![265, 666, 741, 800, 848, 887, 920, 945, 957]
        );
    }

    // All 0 values except arrays
    #[test]
    fn sample2() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s2.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 0);
        assert_eq!(test[0].average_maxrgb, 0);
        assert_eq!(test[0].maxscl, vec![0, 0, 0]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![3, 14024, 43, 56, 219, 1036, 2714, 4668, 14445]
        );
        assert_eq!(test[0].knee_x, 0);
        assert_eq!(test[0].knee_y, 0);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![265, 666, 741, 800, 848, 887, 920, 945, 957]
        );
    }

    // Some small values
    #[test]
    fn sample3() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s3.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 0);
        assert_eq!(test[0].average_maxrgb, 12);
        assert_eq!(test[0].maxscl, vec![0, 1, 0]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![3, 14024, 43, 56, 219, 1036, 2714, 4668, 14445]
        );
        assert_eq!(test[0].knee_x, 0);
        assert_eq!(test[0].knee_y, 0);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![265, 666, 741, 800, 848, 887, 920, 945, 957]
        );
    }

    // More random values
    #[test]
    fn sample4() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s4.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 10);
        assert_eq!(test[0].average_maxrgb, 1);
        assert_eq!(test[0].maxscl, vec![0, 1, 0]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![0, 14024, 43, 56, 219, 0, 2714, 4668, 14445]
        );
        assert_eq!(test[0].knee_x, 1);
        assert_eq!(test[0].knee_y, 0);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![0, 666, 741, 0, 848, 887, 920, 945, 957]
        );
    }

    // Some 0 values except targeted display maximum luminance
    #[test]
    fn sample5() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s5.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 500);
        assert_eq!(test[0].average_maxrgb, 0);
        assert_eq!(test[0].maxscl, vec![0, 0, 0]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![0, 0, 100, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(test[0].knee_x, 0);
        assert_eq!(test[0].knee_y, 0);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![102, 205, 307, 410, 512, 614, 717, 819, 922]
        );
    }

    // More random values
    #[test]
    fn sample6() {
        let mut test: Vec<Metadata> = Vec::new();
        match parse_metadata("./assets/ToS-s6.h265", false) {
            Ok(vec) => test = llc_read_metadata(vec),
            Err(e) => println!("{}", e),
        }

        assert_eq!(test[0].num_windows, 1);
        assert_eq!(test[0].targeted_system_display_maximum_luminance, 500);
        assert_eq!(test[0].average_maxrgb, 1);
        assert_eq!(test[0].maxscl, vec![1, 3, 6]);
        assert_eq!(
            test[0].distribution_index,
            vec![1, 5, 10, 25, 50, 75, 90, 95, 99]
        );
        assert_eq!(
            test[0].distribution_values,
            vec![0, 0, 100, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(test[0].knee_x, 2048);
        assert_eq!(test[0].knee_y, 85);
        assert_eq!(
            test[0].bezier_curve_data,
            vec![102, 205, 307, 410, 512, 614, 717, 819, 922]
        );
    }
}
