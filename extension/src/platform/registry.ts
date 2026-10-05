import type { PlatformAdapter } from './adapter';
import { CodeforcesAdapter } from './codeforces';
import { LeetCodeAdapter } from './leetcode';
import { CodeChefAdapter } from './codechef';
import {
  AtCoderAdapter,
  HackerRankAdapter,
  GeeksforGeeksAdapter,
  HackerEarthAdapter,
  CsesAdapter,
} from './competitive_platforms';
import {
  OnlineGdbAdapter,
  JDoodleAdapter,
  ProgramizAdapter,
  ReplitAdapter,
} from './online_ides';

export const allAdapters: PlatformAdapter[] = [
  new CodeforcesAdapter(),
  new LeetCodeAdapter(),
  new CodeChefAdapter(),
  new AtCoderAdapter(),
  new HackerRankAdapter(),
  new GeeksforGeeksAdapter(),
  new HackerEarthAdapter(),
  new CsesAdapter(),
  new OnlineGdbAdapter(),
  new JDoodleAdapter(),
  new ProgramizAdapter(),
  new ReplitAdapter(),
];

export function getAdapterForUrl(url: string): PlatformAdapter | null {
  for (const adapter of allAdapters) {
    if (adapter.isMatch(url)) {
      return adapter;
    }
  }
  return null;
}

export const getAllPlatformAdapters = () => allAdapters;
export const getPlatformAdapter = getAdapterForUrl;

