use crate::pd;

#[derive(Debug)]
pub struct CProductDescriptionForClient {
    _Files: CBNPFileSet,          // read_struct
    _Categories: CBNPCategorySet, // read_struct
}

impl CProductDescriptionForClient {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> CProductDescriptionForClient {
        CProductDescriptionForClient {
            _Files: pdr.read_struct("_Files"),
            _Categories: CBNPCategorySet {},
        }
    }
}

#[derive(Debug)]
pub struct CBNPFileSet {
    pub _Files: Vec<CBNPFile>, // read_struct_vec
}

impl pd::Readable for CBNPFileSet {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFileSet {
        let mut files: Vec<CBNPFile> = Vec::new();
        while pdr.has_named_struct("_Files") {
            pdr.read_struct_begin("_Files");
            files.push(CBNPFile::from(pdr));
            pdr.read_struct_end("_Files");
        }

        CBNPFileSet { _Files: files }
    }
}

#[derive(Debug)]
pub struct CBNPCategorySet {}

#[derive(Debug)]
pub struct CBNPFile {
    pub _FileName: String,               // read_prop
    pub _Versions: Vec<CBNPFileVersion>, // read_struct_vec
}

impl CBNPFile {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> CBNPFile {
        CBNPFile {
            _FileName: pdr.read_string("_FileName"),
            _Versions: pdr.read_struct_vec("_Versions"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPFileVersion {
    pub _VersionNumber: u32,
    pub _FileSize: u32,
    pub _7ZFileSize: u32,
    pub _FileTime: u32,
    pub _PatchSize: u32,
    pub _HashKey: Vec<u32>, // read_prop_vec
}

impl pd::Readable for CBNPFileVersion {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFileVersion {
        CBNPFileVersion {
            _VersionNumber: pdr.read_u32("_VersionNumber"),
            _FileSize: pdr.read_u32("_FileSize"),
            _7ZFileSize: pdr.read_u32("_7ZFileSize"),
            _FileTime: pdr.read_u32("_FileTime"),
            _PatchSize: pdr.read_u32("_PatchSize"),
            _HashKey: vec![
                pdr.read_u32("_HashKey"),
                pdr.read_u32("_HashKey"),
                pdr.read_u32("_HashKey"),
                pdr.read_u32("_HashKey"),
                pdr.read_u32("_HashKey"),
            ], // read_prop_vec
        }
    }
}
