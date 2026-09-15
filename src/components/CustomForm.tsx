import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Input } from '@/components/ui/input';
import type { Mention } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

type Props = {
  packMention: Mention | undefined;
  onCancel: () => void;
  onSubmit: (value: {
    label: string;
    insert: string;
    mention: boolean;
    mentionText?: string;
    mentionHtml?: string;
  }) => void;
};

export default function CustomForm({ packMention, onCancel, onSubmit }: Props) {
  const [label, setLabel] = useState('');
  const [insert, setInsert] = useState('');
  const [withMention, setWithMention] = useState(true);
  const [captured, setCaptured] = useState<Mention | null>(null);
  const [notice, setNotice] = useState('');

  const mention = captured ?? packMention;
  const ready = label.trim() && insert.trim();

  const capture = async () => {
    const clipboard = await invoke<{
      text: string;
      html: string | null;
    } | null>('read_clipboard_mention');

    if (!clipboard?.text) {
      setNotice('Clipboard is empty.');
      return;
    }

    setCaptured({ text: clipboard.text, html: clipboard.html ?? undefined });
    setWithMention(true);
    setNotice(
      clipboard.html ? '' : 'Plain text only — may not become a real mention.',
    );
  };

  return (
    <div className="border-hairline flex flex-col gap-2 border-t px-5 py-3">
      <Input
        autoFocus
        value={label}
        onChange={(event) => setLabel(event.target.value)}
        placeholder="Label"
      />

      <div className="border-hairline bg-input flex items-center gap-2 rounded-md border px-2.5 py-1.5">
        {mention && withMention ? (
          <span className="shrink-0 rounded bg-white/15 px-1.5 py-0.5 font-mono text-xs text-white/70">
            {mention.text}
          </span>
        ) : null}
        <input
          value={insert}
          onChange={(event) => setInsert(event.target.value)}
          placeholder="--model opus --effort high"
          className="min-w-0 flex-1 bg-transparent font-mono text-xs text-white/90 outline-none placeholder:text-white/30"
        />
      </div>

      <div className="flex items-center justify-between gap-2">
        <label className="text-muted-foreground flex items-center gap-2 text-xs">
          <Checkbox
            checked={withMention}
            disabled={!mention}
            onCheckedChange={(value) => setWithMention(value === true)}
          />
          Prepend mention
        </label>

        <Button variant="outline" size="sm" onClick={() => void capture()}>
          Use mention from clipboard
        </Button>
      </div>

      {notice ? <p className="text-xs text-amber-300/70">{notice}</p> : null}

      <div className="flex justify-end gap-2">
        <Button variant="ghost" size="sm" onClick={onCancel}>
          Cancel
        </Button>
        <Button
          size="sm"
          disabled={!ready}
          onClick={() =>
            onSubmit({
              label: label.trim(),
              insert: insert.trim(),
              mention: withMention && Boolean(mention),
              mentionText: captured?.text,
              mentionHtml: captured?.html,
            })
          }
        >
          Add
        </Button>
      </div>
    </div>
  );
}
