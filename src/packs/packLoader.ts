import defaults from "../../packs/default.json";
import github from "../../packs/github.json";
import jira from "../../packs/jira.json";
import slack from "../../packs/slack.json";
import type { Pack } from "../types";

export const DEFAULT_PACK_ID = "default";

export const PACKS: Pack[] = [jira, github, slack, defaults] as Pack[];
