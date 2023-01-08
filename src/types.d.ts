export interface Apps {
  SPApplicationsDataType: SPApplicationsDataType[];
}

export interface SPApplicationsDataType {
  _name: string;
  arch_kind: ArchKind;
  lastModified: string;
  obtained_from: ObtainedFrom;
  path: string;
  signed_by?: string[];
  version?: string;
  info?: string;
}

export enum ArchKind {
  ArchArm = "arch_arm",
  ArchArmI64 = "arch_arm_i64",
  ArchI64 = "arch_i64",
  ArchOther = "arch_other",
}

export enum ObtainedFrom {
  Apple = "apple",
  IdentifiedDeveloper = "identified_developer",
  MACAppStore = "mac_app_store",
  Unknown = "unknown",
}
