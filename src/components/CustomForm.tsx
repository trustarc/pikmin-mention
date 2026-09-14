import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Mention } from "../types";

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
  const [label, setLabel] = useState("");
  const [insert, setInsert] = useState("");
  const [withMention, setWithMention] = useState(true);
  const [captured, setCaptured] = useState<Mention | null>(null);
  const [notice, setNotice] = useState("");

  const mention = captured ?? packMention;
  const ready = label.trim() && insert.trim();

  const capture = async () => {
    const clipboard = await invoke<{ text: string; html: string | null } | null>(
      "read_clipboard_mention",
    );

    if (!clipboard?.text) {
      setNotice("Clipboard is empty.");
      return;
    }

    setCaptured({ text: clipboard.text, html: clipboard.html ?? undefined });
    setWithMention(true);
    setNotice(clipboard.html ? "" : "Plain text only — may not become a real mention.");
  };

  return (
    <div className="flex flex-col gap-2 border-t border-hairline px-5 py-3">
      <input
        autoFocus
        value={label}
        onChange={(event) => setLabel(event.target.value)}
        placeholder="Label"
        className="rounded-md border border-hairline bg-white/5 px-2.5 py-1.5 text-sm text-white/90 outline-none placeholder:text-white/30"
      />

      <div className="flex items-center gap-2 rounded-md border border-hairline bg-white/5 px-2.5 py-1.5">
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
        <label className="flex cursor-pointer items-center gap-2 text-xs text-white/45">
          <input
            type="checkbox"
            checked={withMention}
            disabled={!mention}
            onChange={(event) => setWithMention(event.target.checked)}
            className="accent-white/60"
          />
          Prepend mention
        </label>

        <button
          type="button"
          onClick={() => void capture()}
          className="rounded-md border border-hairline px-2 py-1 text-xs text-white/50 hover:bg-white/10 hover:text-white/80"
        >
          Use mention from clipboard
        </button>
      </div>

      {notice ? <p className="text-xs text-amber-300/70">{notice}</p> : null}

      <div className="flex justify-end gap-2">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-md px-2.5 py-1 text-xs text-white/50 hover:text-white/80"
        >
          Cancel
        </button>
        <button
          type="button"
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
          className="rounded-md border border-hairline bg-white/10 px-2.5 py-1 text-xs text-white/85 disabled:opacity-30 hover:bg-white/15"
        >
          Add
        </button>
      </div>
    </div>
  );
}
