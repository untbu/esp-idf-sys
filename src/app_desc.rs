#[macro_export]
macro_rules! esp_app_desc {
    () => {
        // For backwards compatibility
        $crate::esp_app_desc!(false);
    };
    ($fix_date_time_swap: expr) => {
        #[no_mangle]
        #[used]
        #[link_section = ".rodata_desc"]
        #[allow(non_upper_case_globals)]
        pub static esp_app_desc: $crate::esp_app_desc_t = {
            const fn str_to_cstr_array<const C: usize>(s: &str) -> [::core::ffi::c_char; C] {
                let mut ret: [::core::ffi::c_char; C] = [0; C];

                let mut i = 0;
                while i < C {
                    if i < s.len() {
                        ret[i] = s.as_bytes()[i] as _;
                    } else {
                        break;
                    }

                    i += 1;
                }

                ret
            }

            $crate::esp_app_desc_t {
                magic_word: $crate::ESP_APP_DESC_MAGIC_WORD,
                secure_version: 0,
                reserv1: [0; 2],
                version: str_to_cstr_array(env!("CARGO_PKG_VERSION")),
                project_name: str_to_cstr_array(env!("CARGO_PKG_NAME")),
                #[cfg(all(esp_idf_app_compile_time_date, not(esp_idf_app_reproducible_build)))]
                time: str_to_cstr_array(if $fix_date_time_swap {
                    $crate::build_time::build_time_utc!("%H:%M:%S")
                } else {
                    $crate::build_time::build_time_utc!("%Y-%m-%d")
                }),
                #[cfg(all(esp_idf_app_compile_time_date, not(esp_idf_app_reproducible_build)))]
                date: str_to_cstr_array(if $fix_date_time_swap {
                    $crate::build_time::build_time_utc!("%Y-%m-%d")
                } else {
                    $crate::build_time::build_time_utc!("%H:%M:%S")
                }),
                #[cfg(not(all(
                    esp_idf_app_compile_time_date,
                    not(esp_idf_app_reproducible_build)
                )))]
                time: [0i8; 16],
                #[cfg(not(all(
                    esp_idf_app_compile_time_date,
                    not(esp_idf_app_reproducible_build)
                )))]
                date: [0i8; 16],
                idf_ver: str_to_cstr_array($crate::const_format::formatcp!(
                    "{}.{}.{}",
                    $crate::ESP_IDF_VERSION_MAJOR,
                    $crate::ESP_IDF_VERSION_MINOR,
                    $crate::ESP_IDF_VERSION_PATCH
                )),
                app_elf_sha256: [0; 32],
                #[cfg(not(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version_full = "5.2.0",
                    esp_idf_version_full = "5.2.1",
                    esp_idf_version_full = "5.2.2",
                    esp_idf_version_full = "5.3.0",
                    esp_idf_version_full = "5.3.1"
                )))]
                min_efuse_blk_rev_full: 0,
                #[cfg(not(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version_full = "5.2.0",
                    esp_idf_version_full = "5.2.1",
                    esp_idf_version_full = "5.2.2",
                    esp_idf_version_full = "5.3.0",
                    esp_idf_version_full = "5.3.1"
                )))]
                max_efuse_blk_rev_full: 0,
                #[cfg(not(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version = "5.2",
                    esp_idf_version = "5.3"
                )))]
                mmu_page_size: 0,
                #[cfg(not(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version = "5.2",
                    esp_idf_version = "5.3"
                )))]
                reserv3: [0; 4],
                #[cfg(not(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version = "5.2",
                    esp_idf_version = "5.3"
                )))]
                reserv2: [0; 18],
                #[cfg(any(esp_idf_version_full = "5.2.3", esp_idf_version_full = "5.3.2"))]
                reserv2: [0; 19],
                #[cfg(any(
                    esp_idf_version_major = "4",
                    esp_idf_version = "5.0",
                    esp_idf_version = "5.1",
                    esp_idf_version_full = "5.2.0",
                    esp_idf_version_full = "5.2.1",
                    esp_idf_version_full = "5.2.2",
                    esp_idf_version_full = "5.3.0",
                    esp_idf_version_full = "5.3.1"
                ))]
                reserv2: [0; 20],
            }
        };
    };
}
