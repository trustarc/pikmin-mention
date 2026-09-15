import { resolveContext } from '@/context/resolveContext';
import type { ActiveContext } from '@/types';
import { describe, expect, test } from 'bun:test';

function browsing(hostname: string | null): ActiveContext {
  return {
    app: 'Google Chrome',
    bundleId: 'com.google.Chrome',
    browser: 'chrome',
    hostname,
  };
}

function running(app: string): ActiveContext {
  return { app, bundleId: '', browser: null, hostname: null };
}

describe('hostname matching', () => {
  test('matches exact domain and subdomains', () => {
    expect(resolveContext(browsing('github.com'))?.id).toBe('github');
    expect(resolveContext(browsing('gist.github.com'))?.id).toBe('github');
    expect(resolveContext(browsing('trustarc.atlassian.net'))?.id).toBe('jira');
  });

  test('rejects lookalike domains', () => {
    expect(resolveContext(browsing('github.com.example.com'))?.id).toBe(
      'default',
    );
    expect(resolveContext(browsing('evilgithub.com'))?.id).toBe('default');
    expect(resolveContext(browsing('notatlassian.net'))?.id).toBe('default');
  });
});

describe('app matching', () => {
  test('matches a desktop app by name', () => {
    expect(resolveContext(running('Slack'))?.id).toBe('slack');
  });

  test('does not match a prefixed name', () => {
    expect(resolveContext(running('Slackware'))?.id).toBe('default');
  });
});

describe('fallback', () => {
  test('falls back to the default pack', () => {
    expect(resolveContext(browsing(null))?.id).toBe('default');
    expect(resolveContext(running('Preview'))?.id).toBe('default');
  });

  test('returns null without context', () => {
    expect(resolveContext(null)).toBeNull();
  });
});
