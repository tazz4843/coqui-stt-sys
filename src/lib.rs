mod bindings;

pub use bindings::*;

#[cfg(test)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(deref_nullptr)]
mod tests {
    use super::*;

    #[test]
    fn bindgen_test_layout_TokenMetadata() {
        assert_eq!(
            ::std::mem::size_of::<TokenMetadata>(),
            16usize,
            concat!("Size of: ", stringify!(TokenMetadata))
        );
        assert_eq!(
            ::std::mem::align_of::<TokenMetadata>(),
            8usize,
            concat!("Alignment of ", stringify!(TokenMetadata))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<TokenMetadata>())).text as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(TokenMetadata),
                "::",
                stringify!(text)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<TokenMetadata>())).timestep as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(TokenMetadata),
                "::",
                stringify!(timestep)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<TokenMetadata>())).start_time as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(TokenMetadata),
                "::",
                stringify!(start_time)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_CandidateTranscript() {
        assert_eq!(
            ::std::mem::size_of::<CandidateTranscript>(),
            24usize,
            concat!("Size of: ", stringify!(CandidateTranscript))
        );
        assert_eq!(
            ::std::mem::align_of::<CandidateTranscript>(),
            8usize,
            concat!("Alignment of ", stringify!(CandidateTranscript))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CandidateTranscript>())).tokens as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(CandidateTranscript),
                "::",
                stringify!(tokens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<CandidateTranscript>())).num_tokens as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(CandidateTranscript),
                "::",
                stringify!(num_tokens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<CandidateTranscript>())).confidence as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(CandidateTranscript),
                "::",
                stringify!(confidence)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Metadata() {
        assert_eq!(
            ::std::mem::size_of::<Metadata>(),
            16usize,
            concat!("Size of: ", stringify!(Metadata))
        );
        assert_eq!(
            ::std::mem::align_of::<Metadata>(),
            8usize,
            concat!("Alignment of ", stringify!(Metadata))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Metadata>())).transcripts as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Metadata),
                "::",
                stringify!(transcripts)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Metadata>())).num_transcripts as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Metadata),
                "::",
                stringify!(num_transcripts)
            )
        );
    }
}
