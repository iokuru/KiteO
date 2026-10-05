import type { SupportedLanguage, ConstraintStatus } from '../shared/types';

export interface PlatformAdapter {
  id: string;
  name: string;
  isMatch(url: string): boolean;
  isEditorAvailable(): boolean;
  getSource(): string | null;
  getLanguage(): SupportedLanguage | null;
  getConstraints?(): string | null;
  evaluateConstraints?(tc: string): ConstraintStatus;
  isLiveContest?(): boolean;
}
