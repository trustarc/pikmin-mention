import CustomForm from '@/components/CustomForm';
import PackTabs from '@/components/PackTabs';
import SnippetList from '@/components/SnippetList';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { resolveContext } from '@/context/resolveContext';
import { formatHotkey, toShortcut } from '@/hotkey';
import { PACKS } from '@/packs/packLoader';
import type { ActiveContext, Settings, Snippet } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { writeHtml, writeText } from '@tauri-apps/plugin-clipboard-manager';
import { relaunch } from '@tauri-apps/plugin-process';
import { check } from '@tauri-apps/plugin-updater';
import { Plus, RotateCcw } from 'lucide-react';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

export default function App() {
  const [context, setContext] = useState<ActiveContext | null>(null);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [adding, setAdding] = useState(false);
  const [defaultHotkey, setDefaultHotkey] = useState('');
  const [mac, setMac] = useState(true);
  const [update, setUpdate] = useState('');
  const [followSelection, setFollowSelection] = useState(false);
  const searchRef = useRef<HTMLInputElement>(null);
  const [query, setQuery] = useState('');
  const [pendingId, setPendingId] = useState<string | null>(null);
  const [recording, setRecording] = useState(false);
  const [error, setError] = useState('');
  const [trusted, setTrusted] = useState(true);

  const matchedId = useMemo(
    () => resolveContext(context)?.id ?? null,
    [context],
  );

  useEffect(() => {
    void invoke<boolean>('accessibility_status').then(setTrusted);
    void invoke<Settings>('get_settings').then(setSettings);
    void invoke<string>('default_hotkey').then(setDefaultHotkey);
    void invoke<boolean>('is_macos').then(setMac);
    void invoke<ActiveContext>('get_active_context').then(setContext);

    const unlistenUpdate = listen('check-update', () => {
      setUpdate('Checking for updates…');

      void check()
        .then(async (found) => {
          if (!found) {
            setUpdate('You are on the latest version.');
            return;
          }

          setUpdate(`Downloading ${found.version}…`);
          await found.downloadAndInstall();
          setUpdate('Restarting…');
          await relaunch();
        })
        .catch((reason) => setUpdate(`Update failed: ${reason}`));
    });

    const unlistenContext = listen<ActiveContext>('context', (event) => {
      setContext(event.payload);
      setQuery('');
      setAdding(false);
      setUpdate('');
    });

    const unlistenFocus = getCurrentWindow().onFocusChanged(
      ({ payload: focused }) => {
        if (focused) {
          searchRef.current?.focus();
          searchRef.current?.select();
          setPendingId(null);
        }
      },
    );

    return () => {
      void unlistenContext.then((off) => off());
      void unlistenUpdate.then((off) => off());
      void unlistenFocus.then((off) => off());
    };
  }, []);

  useEffect(() => {
    setActiveId(matchedId);
  }, [matchedId]);

  const pack = useMemo(
    () => PACKS.find((item) => item.id === activeId) ?? null,
    [activeId],
  );

  const snippets = useMemo(() => {
    if (!pack) {
      return [];
    }

    const custom = (settings?.custom?.[pack.id] ?? []).map((entry) => ({
      id: entry.id,
      label: entry.label,
      category: 'Custom',
      insert: entry.insert,
      mention: entry.mention,
      mentionText: entry.mentionText,
      mentionHtml: entry.mentionHtml,
      custom: true,
    }));

    const all = [...pack.snippets, ...custom];
    const needle = query.trim().toLowerCase();
    const filtered = needle
      ? all.filter((snippet) =>
          [snippet.label, snippet.category]
            .join(' ')
            .toLowerCase()
            .includes(needle),
        )
      : all;

    const pinned = settings?.pinned ?? [];
    const rank = (id: string) => {
      const index = pinned.indexOf(id);
      return index === -1 ? Number.MAX_SAFE_INTEGER : index;
    };

    return [...filtered].sort((a, b) => rank(a.id) - rank(b.id));
  }, [pack, query, settings]);

  const selectedId = useMemo(() => {
    if (pendingId && snippets.some((item) => item.id === pendingId)) {
      return pendingId;
    }

    const preferred = pack ? settings?.lastUsed?.[pack.id] : undefined;
    if (preferred && snippets.some((item) => item.id === preferred)) {
      return preferred;
    }

    return snippets[0]?.id ?? null;
  }, [pendingId, snippets, pack, settings]);

  const activate = useCallback(
    async (snippet: Snippet) => {
      const own = snippet.mentionText
        ? { text: snippet.mentionText, html: snippet.mentionHtml }
        : undefined;
      const mention = snippet.mention ? (own ?? pack?.mention) : undefined;
      const body = snippet.insert;
      const text = mention ? `${mention.text} ${body}` : body;

      try {
        if (mention?.html) {
          await writeHtml(`${mention.html} ${body}`, text);
        } else {
          await writeText(text);
        }
      } catch (reason) {
        setError(`Could not write to the clipboard: ${reason}`);
        return;
      }

      await getCurrentWindow().hide();

      if (pack) {
        void invoke<Settings>('set_last_used', {
          packId: pack.id,
          id: snippet.id,
        })
          .then(setSettings)
          .catch(() => undefined);
      }

      try {
        await invoke('insert_snippet');
      } catch (reason) {
        if (String(reason).includes('accessibility')) {
          setTrusted(false);
        } else {
          setError(String(reason));
        }
        await invoke('dismiss').catch(() => undefined);
      }
    },
    [pack],
  );

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (recording) {
        event.preventDefault();
        if (event.key === 'Escape') {
          setRecording(false);
          return;
        }
        const next = toShortcut(event);
        if (!next) {
          return;
        }
        setRecording(false);
        invoke<Settings>('set_hotkey', { hotkey: next })
          .then((next) => {
            setSettings(next);
            setError('');
          })
          .catch((reason) => setError(String(reason)));
        return;
      }

      if (event.key === 'Escape') {
        if (adding) {
          setAdding(false);
          return;
        }
        void invoke('dismiss');
        return;
      }

      if (adding) {
        return;
      }

      if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
        event.preventDefault();
        const step = event.key === 'ArrowDown' ? 1 : -1;

        const index = snippets.findIndex((item) => item.id === selectedId);
        const next =
          snippets[(index + step + snippets.length) % snippets.length];
        setFollowSelection(true);
        setPendingId(next?.id ?? null);
        return;
      }

      if (event.key === 'Tab') {
        event.preventDefault();
        const index = PACKS.findIndex((item) => item.id === activeId);
        const step = event.shiftKey ? -1 : 1;
        const at =
          index === -1 ? 0 : (index + step + PACKS.length) % PACKS.length;
        setActiveId(PACKS[at].id);
        return;
      }

      const digit = /^Digit([1-9])$/.exec(event.code);
      if (digit && (event.metaKey || event.ctrlKey || event.altKey)) {
        event.preventDefault();
        const target = snippets[Number(digit[1]) - 1];
        if (target) {
          void activate(target);
        }
        return;
      }

      if (event.key === 'Enter') {
        event.preventDefault();
        const selected = snippets.find((item) => item.id === selectedId);
        if (selected) {
          void activate(selected).catch((reason) => setError(String(reason)));
        }
      }
    };

    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [recording, adding, snippets, selectedId, activeId, activate]);

  return (
    <main className="border-hairline bg-surface flex h-full flex-col overflow-hidden rounded-2xl border font-sans text-base text-white/95 backdrop-blur-2xl">
      <header className="flex items-center justify-between gap-3 px-5 pt-4 pb-3">
        <div className="flex min-w-0 flex-col">
          <span className="text-sm font-semibold tracking-tight">
            Pikmin Mention
          </span>
          <span className="truncate text-xs text-white/35">
            {context?.hostname ?? context?.app ?? 'No app detected'}
          </span>
        </div>
        <div className="flex items-center gap-1">
          {!recording &&
          settings?.hotkey &&
          defaultHotkey &&
          settings.hotkey !== defaultHotkey ? (
            <Button
              variant="ghost"
              size="icon"
              aria-label="Reset snippet"
              title={`Reset to ${formatHotkey(defaultHotkey, mac)}`}
              onClick={() => {
                void invoke<Settings>('reset_hotkey')
                  .then((next) => {
                    setSettings(next);
                    setError('');
                  })
                  .catch((reason) => setError(String(reason)));
              }}
              className="size-7 text-white/35"
            >
              <RotateCcw className="size-3.5" />
            </Button>
          ) : null}
          <Button
            variant="outline"
            size="sm"
            onClick={() => setRecording(true)}
            className="h-7 font-mono text-xs text-white/70"
          >
            {recording
              ? 'Press keys…'
              : formatHotkey(settings?.hotkey ?? '', mac)}
          </Button>
        </div>
      </header>

      <PackTabs
        packs={PACKS}
        activeId={activeId}
        matchedId={matchedId}
        onSelect={setActiveId}
      />

      <Input
        ref={searchRef}
        autoFocus
        value={query}
        onChange={(event) => setQuery(event.target.value)}
        placeholder="Search snippets"
        className="border-hairline rounded-none border-0 border-b bg-transparent px-5 py-2.5 focus-visible:ring-0"
      />

      <div className="flex-1 overflow-y-auto px-3 py-3">
        {!pack ? (
          <div className="flex h-full flex-col items-center justify-center gap-1 px-6 text-center">
            <p className="text-sm text-white/45">No pack for this app.</p>
            <p className="text-xs text-white/25">
              {context?.app
                ? `${context.app} is not recognised`
                : 'Unknown app'}{' '}
              — pick a tab above.
            </p>
          </div>
        ) : snippets.length ? (
          <SnippetList
            snippets={snippets}
            selectedId={selectedId}
            pinned={settings?.pinned ?? []}
            mac={mac}
            followSelection={followSelection}
            onActivate={activate}
            onHover={() => undefined}
            onTogglePin={(snippet) => {
              void invoke<Settings>('toggle_pin', { id: snippet.id }).then(
                setSettings,
              );
            }}
            onReorder={(id, toIndex) => {
              const pinned = (settings?.pinned ?? []).filter(
                (value) => value !== id,
              );
              pinned.splice(Math.min(toIndex, pinned.length), 0, id);
              void invoke<Settings>('set_pinned', { pinned }).then(setSettings);
            }}
            onRemove={(snippet) => {
              void invoke<Settings>('remove_custom', {
                packId: pack.id,
                id: snippet.id,
              }).then(setSettings);
            }}
          />
        ) : (
          <p className="py-6 text-center text-sm text-white/35">
            {query.trim() ? 'No match.' : 'No snippets yet.'}
          </p>
        )}
      </div>

      {pack && adding ? (
        <CustomForm
          packMention={pack.mention}
          onCancel={() => setAdding(false)}
          onSubmit={(value) => {
            setAdding(false);
            void invoke<Settings>('add_custom', {
              packId: pack.id,
              label: value.label,
              insert: value.insert,
              mention: value.mention,
              mentionText: value.mentionText ?? null,
              mentionHtml: value.mentionHtml ?? null,
            }).then(setSettings);
          }}
        />
      ) : pack ? (
        <Button
          variant="ghost"
          onClick={() => setAdding(true)}
          className="border-hairline h-auto w-full justify-start rounded-none border-t px-5 py-2.5 text-xs text-white/40"
        >
          <Plus className="size-3.5" />
          Add custom
        </Button>
      ) : null}

      {!trusted ? (
        <div className="border-hairline flex items-center justify-between gap-3 border-t px-5 py-2.5">
          <span className="text-xs text-white/45">
            Copied to clipboard. Enable Accessibility to paste automatically.
          </span>
          <Button
            size="sm"
            onClick={() => {
              void invoke<boolean>('request_accessibility').then(setTrusted);
            }}
          >
            Grant access
          </Button>
        </div>
      ) : null}

      {update ? (
        <p className="border-hairline border-t px-5 py-2.5 text-xs text-white/60">
          {update}
        </p>
      ) : null}

      {error ? <p className="px-5 pb-3 text-xs text-red-400">{error}</p> : null}
    </main>
  );
}
