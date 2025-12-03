use crate::pd;

#[derive(Debug)]
pub struct CProductDescriptionForClient {
    _Files: CBNPFileSet,          // read_struct
    _Categories: CBNPCategorySet, // read_struct
}

impl CProductDescriptionForClient {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> CProductDescriptionForClient {
        let files: CBNPFileSet = pdr.read_struct("_Files");
        //let categories: CBNPCategorySet = pdr.read_struct("_Categories");
        CProductDescriptionForClient {
            _Files: files,
            _Categories: CBNPCategorySet { _Category: vec![] },
        }
    }
}

#[derive(Debug)]
pub struct CBNPFileSet {
    pub _Files: Vec<CBNPFile>, // read_struct_vec
}

impl pd::Readable for CBNPFileSet {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFileSet {
        CBNPFileSet {
            _Files: pdr.read_struct_vec("_Files"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPCategorySet {
    pub _Category: Vec<CBNPCategory>,
}

impl pd::Readable for CBNPCategorySet {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPCategorySet {
        CBNPCategorySet {
            _Category: pdr.read_struct_vec("_Category"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPCategory {
    pub _Name: String,
    pub _IsOptional: i32,
    pub _Files: Vec<String>,
}

impl pd::Readable for CBNPCategory {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPCategory {
        CBNPCategory {
            _Name: pdr.read_prop("_Name"),
            _IsOptional: pdr.read_prop("_IsOptional"),
            _Files: pdr.read_prop_vec("_Files"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPFile {
    pub _FileName: String,               // read_prop
    pub _Versions: Vec<CBNPFileVersion>, // read_struct_vec
}

impl pd::Readable for CBNPFile {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFile {
        CBNPFile {
            _FileName: pdr.read_prop("_FileName"),
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
            _VersionNumber: pdr.read_prop("_VersionNumber"),
            _FileSize: pdr.read_prop("_FileSize"),
            _7ZFileSize: pdr.read_prop("_7ZFileSize"),
            _FileTime: pdr.read_prop("_FileTime"),
            _PatchSize: pdr.read_prop("_PatchSize"),
            _HashKey: pdr.read_prop_vec("_HashKey"), // read_prop_vec
        }
    }
}
