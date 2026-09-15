export type Mention = {
  text: string;
  html?: string;
};

export type Snippet = {
  id: string;
  keys: string[];
  label: string;
  category: string;
  insert?: string;
  mention?: boolean;
  mentionText?: string;
  mentionHtml?: string;
  custom?: boolean;
};

export type Pack = {
  id: string;
  name: string;
  version: number;
  match: { domains: string[]; apps: string[] };
  mention?: Mention;
  snippets: Snippet[];
};

export type ActiveContext = {
  app: string;
  bundleId: string;
  browser: string | null;
  hostname: string | null;
};

export type CustomSnippet = {
  id: string;
  label: string;
  insert: string;
  mention: boolean;
  mentionText?: string;
  mentionHtml?: string;
};

export type Settings = {
  hotkey: string;
  pinned: string[];
  custom: Record<string, CustomSnippet[]>;
  lastUsed: Record<string, string>;
};
