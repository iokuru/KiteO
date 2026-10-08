export type SupportedLanguage = 'cpp' | 'java';

export type AllowedAlgorithm = string;

export type ConstraintStatus = 'Within limits' | 'Likely too slow' | "Can't tell";

export interface AnalysisResult {
  tc: string;
  sc: string;
  algorithms: AllowedAlgorithm[];
  constraintStatus?: ConstraintStatus;
}

export interface PlatformAdapter {
  id: string;
  name: string;
  isMatch(url: string): boolean;
  isEditorAvailable(): boolean;
  getSource(): string | null;
  getLanguage(): SupportedLanguage | null;
  getConstraints?(): string | null;
  isLiveContest?(): boolean;
}
