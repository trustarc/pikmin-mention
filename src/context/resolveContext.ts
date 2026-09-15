import { DEFAULT_PACK_ID, PACKS } from '../packs/packLoader';
import type { ActiveContext, Pack } from '../types';

function matchesDomain(hostname: string, domain: string) {
  const target = domain.toLowerCase();
  return hostname === target || hostname.endsWith(`.${target}`);
}

function matchesApp(context: ActiveContext, app: string) {
  return context.app.toLowerCase() === app.toLowerCase();
}

export function resolveContext(context: ActiveContext | null): Pack | null {
  if (!context) {
    return null;
  }

  if (context.hostname) {
    const byDomain = PACKS.find((pack) =>
      pack.match.domains.some((domain) =>
        matchesDomain(context.hostname!, domain),
      ),
    );
    if (byDomain) {
      return byDomain;
    }
  }

  const byApp = PACKS.find((pack) =>
    pack.match.apps.some((app) => matchesApp(context, app)),
  );
  if (byApp) {
    return byApp;
  }

  return PACKS.find((pack) => pack.id === DEFAULT_PACK_ID) ?? null;
}
