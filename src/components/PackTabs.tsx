import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import type { Pack } from '@/types';

type Props = {
  packs: Pack[];
  activeId: string | null;
  matchedId: string | null;
  onSelect: (id: string) => void;
};

export default function PackTabs({
  packs,
  activeId,
  matchedId,
  onSelect,
}: Props) {
  return (
    <Tabs
      value={activeId ?? ''}
      onValueChange={onSelect}
      activationMode="manual"
      className="border-hairline border-b px-3 py-2"
    >
      <TabsList className="bg-transparent p-0">
        {packs.map((pack) => (
          <TabsTrigger
            key={pack.id}
            value={pack.id}
            className="gap-1.5 border-0 text-white/45 data-[state=active]:bg-white/15 data-[state=active]:text-white"
          >
            {pack.name}
            {pack.id === matchedId ? (
              <span className="size-1 rounded-full bg-emerald-400" />
            ) : null}
          </TabsTrigger>
        ))}
      </TabsList>
    </Tabs>
  );
}
