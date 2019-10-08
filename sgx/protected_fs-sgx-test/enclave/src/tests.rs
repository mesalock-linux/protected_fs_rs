use protected_fs::remove_protected_file;
use protected_fs::ProtectedFile;
use rand_core::RngCore;
use std::io::{Read, Write};

pub fn test_sgxfs_basic() {
    let key = [90u8; 16];

    let mut write_data: [u8; 16] = [0; 16];
    let mut read_data: [u8; 16] = [0; 16];
    let write_size;
    let read_size;
    {
        let mut rng = rdrand::RdRand::new().unwrap();
        rng.fill_bytes(&mut write_data);

        let opt = ProtectedFile::create_ex("sgx_file", &key);
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.write(&write_data);
        assert_eq!(result.is_ok(), true);
        write_size = result.unwrap();
    }

    {
        let opt = ProtectedFile::open_ex("sgx_file", &key);
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.read(&mut read_data);
        assert_eq!(result.is_ok(), true);
        read_size = result.unwrap();
    }

    let result = remove_protected_file("sgx_file");
    assert_eq!(result.is_ok(), true);

    assert_eq!(write_data, read_data);
    assert_eq!(write_size, read_size);

    {
        let opt = ProtectedFile::open_ex("/", &key);
        assert_eq!(opt.is_err(), true);
        let opt = ProtectedFile::open_ex(".", &key);
        assert_eq!(opt.is_err(), true);
        let opt = ProtectedFile::open_ex("..", &key);
        assert_eq!(opt.is_err(), true);
        let opt = ProtectedFile::open_ex("?", &key);
        assert_eq!(opt.is_err(), true);
    }
    {
        let opt = ProtectedFile::open_ex("/dev/isgx", &key);
        assert_eq!(opt.is_ok(), true);
    }
    {
        let opt = ProtectedFile::create_ex("/", &key);
        assert_eq!(opt.is_err(), true);
    }
    {
        let opt = ProtectedFile::create_ex("/proc/100", &key);
        assert_eq!(opt.is_err(), true);
        let opt = ProtectedFile::create_ex(".", &key);
        assert_eq!(opt.is_err(), true);
        let opt = ProtectedFile::create_ex("..", &key);
        assert_eq!(opt.is_err(), true);
    }
}

pub fn test_sgxfs_large_file() {
    const BLOCK_SIZE: usize = 2048; 
    const NBLOCKS: usize = 0x0010_0000; 

    let key = [90u8; 16]; 
    let mut auth_tag = [0u8; 16]; 

    let mut write_data = [0u8; BLOCK_SIZE]; 
    let mut read_data = [0u8; BLOCK_SIZE];

    let mut write_size;
    let mut read_size;

    let mut rng = rdrand::RdRand::new().unwrap();
    rng.fill_bytes(&mut write_data);
    let fname = "large_file";

    {
        let opt = ProtectedFile::create_ex(fname, &key);
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();
        for _i in 0..NBLOCKS {
            let result = file.write(&write_data);
            assert_eq!(result.is_ok(), true);
            write_size = result.unwrap();
            assert_eq!(write_size, write_data.len());
        }

        // Flush before we get the final auth_tag
        let result = file.flush();
        assert_eq!(result.is_ok(), true);

        let result = file.get_current_meta_gmac(&mut auth_tag);
        assert_eq!(result.is_ok(), true);
    }

    {
        let mut auth_tag_in_file = [0xffu8; 16];
        let opt = ProtectedFile::open_ex(fname, &key);
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.get_current_meta_gmac(&mut auth_tag_in_file);
        assert_eq!(result.is_ok(), true);

        assert_eq!(auth_tag_in_file, auth_tag);

        for _i in 0..NBLOCKS {
            let result = file.read(&mut read_data);
            assert_eq!(result.is_ok(), true);
            read_size = result.unwrap();
            assert_eq!(read_size, read_data.len());
        }
    }
    assert_eq!(remove_protected_file(fname).is_ok(), true);
}