use crate::pd;

pub struct CProductDescriptionForClient {
    _Files: CBNPFileSet,          // read_struct
    _Categories: CBNPCategorySet, // read_struct
}

impl CProductDescriptionForClient {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> Option<CProductDescriptionForClient> {
        pdr.read_struct_begin();
        let files = CBNPFileSet::from(pdr);
        //pdr.read_struct_end();
        None
    }
}

pub struct CBNPFileSet {
    pub _Files: Vec<CBNPFile>, // read_struct_vec
}

impl CBNPFileSet {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> Option<CBNPFileSet> {
        let mut files: Vec<Option<CBNPFile>> = Vec::new();
        while pdr.has_struct(16) {
            pdr.read_struct_begin();
            files.push(CBNPFile::from(pdr));
            pdr.read_struct_end();
        }
        None
    }
}

pub struct CBNPCategorySet {}

pub struct CBNPFile {
    pub _FileName: String,               // read_prop
    pub _Versions: Vec<CBNPFileVersion>, // read_struct_vec
}

impl CBNPFile {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> Option<CBNPFile> {
        let file_name = pdr.read_string();
        let versions: Vec<CBNPFileVersion> = Vec::new();
        pdr.read_struct_begin();
        let version = CBNPFileVersion::from(pdr);
        pdr.read_struct_end();
        None
    }
}

pub struct CBNPFileVersion {
    pub _VersionNumber: u32,
    pub _FileSize: u32,
    pub _7ZFileSize: u32,
    pub _FileTime: u32,
    pub _PatchSize: u32,
    pub _HashKey: Vec<u32>, // read_prop_vec
}

impl CBNPFileVersion {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> CBNPFileVersion {
        CBNPFileVersion {
            _VersionNumber: pdr.read_u32(),
            _FileSize: pdr.read_u32(),
            _7ZFileSize: pdr.read_u32(),
            _FileTime: pdr.read_u32(),
            _PatchSize: pdr.read_u32(),
            _HashKey: vec![
                pdr.read_u32(),
                pdr.read_u32(),
                pdr.read_u32(),
                pdr.read_u32(),
                pdr.read_u32(),
            ], // read_prop_vec
        }
    }
}
