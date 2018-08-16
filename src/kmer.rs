use std::fmt;

const MAX_KMER_LENGTH: usize = 32;

#[derive(Debug)]
pub enum KmerError {
    InvalidBaseError(char),
    LengthError(usize),
}

impl fmt::Display for KmerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KmerError::InvalidBaseError(base) =>
                write!(f, "Invalid base character: {}", base),
            KmerError::LengthError(length) =>
                write!(f, "k-mer length ({}) is greater than maximum ({})",
                    length, MAX_KMER_LENGTH),
        }
    }
}

pub fn kmer_to_bits(kmer: &String) -> Result<u64, KmerError> {
    if kmer.len() > MAX_KMER_LENGTH {
        return Err(KmerError::LengthError(kmer.len()));
    }

    let mut bit_repr: u64 = 0;

    for (index, base) in kmer.chars().enumerate() {
        let this_base_bytes: u64 = match base {
            'A' => 0b00,
            'C' => 0b01,
            'G' => 0b10,
            'T' => 0b11,
            _ => return Err(KmerError::InvalidBaseError(base)),
        };

        bit_repr += this_base_bytes << (index*2);
    }

    Ok(bit_repr)
}

pub fn bits_to_kmer(bits: u64, k: usize) -> Result<String, KmerError> {
    if k > MAX_KMER_LENGTH {
        return Err(KmerError::LengthError(k));
    }

    let mut string_repr = String::new();

    for index in 0..k {
        let this_base_bytes = (bits & (0b11_u64 << (index*2))) >> (index*2);
        let base: char = match this_base_bytes {
            0b00 => 'A',
            0b01 => 'C',
            0b10 => 'G',
            0b11 => 'T',
            _ => panic!("Two bits cannot have value outside [0,3]."),
        };
        string_repr.push(base);
    }

    Ok(string_repr)
}

pub fn reverse_complement(sequence: &String) -> Result<String, KmerError> {
    let mut revcomp = String::new();
    for base in sequence.chars().rev() {
        let complement = match base {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => return Err(KmerError::InvalidBaseError(base)),
        };
        revcomp.push(complement);
    }

    Ok(revcomp)
}

pub fn get_canonical_repr(kmer: &String) -> Result<String, KmerError> {
    let revcomp = reverse_complement(kmer)?;

    Ok(if kmer < &revcomp {kmer.clone()} else {revcomp})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_reversible() {
        let kmer_str_1 = String::from("ACTGACTGAC");
        let bits = kmer_to_bits(&kmer_str_1).unwrap();
        let kmer_str_2 = bits_to_kmer(bits, kmer_str_1.len()).unwrap();
        assert_eq!(kmer_str_1, kmer_str_2);
    }

    #[test]
    fn revcomp_reversible() {
        let kmer1 = String::from("ATTTACAGCTATG");
        let kmer2 = reverse_complement(&kmer1).unwrap();
        let kmer3 = reverse_complement(&kmer2).unwrap();
        assert_eq!(kmer1, kmer3);
    }

    #[test]
    fn canonical() {
        let kmer1 = String::from("ATTTACAGCTATG");
        let kmer2 = reverse_complement(&kmer1).unwrap();
        assert_eq!(kmer1, get_canonical_repr(&kmer1).unwrap());
        assert_eq!(kmer1, get_canonical_repr(&kmer2).unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_kmer_base() {
        let kmer_str = String::from("ABCDEFGHIJ");
        kmer_to_bits(&kmer_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_revcomp_base() {
        reverse_complement(&String::from("ATTTACAQCTATG")).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_canonical() {
        get_canonical_repr(&String::from("ATTTACAQCTATG")).unwrap();
    }

    #[test]
    #[should_panic]
    fn too_long_kmer_to_bits() {
        let kmer = String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        kmer_to_bits(&kmer).unwrap();
    }

    #[test]
    #[should_panic]
    fn too_long_bits_to_kmer() {
        bits_to_kmer(1234567, 33).unwrap();
    }
}
