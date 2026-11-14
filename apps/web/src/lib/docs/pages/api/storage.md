`api.storage` keeps data when the widget is unmounted (a setting change, a reload) and when the app restarts. Pick a scope for each value:

| Scope                  | Shared by                                                                                         |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `api.storage.instance` | This placement only: two copies of the widget each have their own (a to-do list, a note).         |
| `api.storage.widget`   | Every placement of this widget (an account, a cache, a preference that should follow every copy). |

```js
const todos = (await api.storage.instance.get('todos')) ?? [];
// ... later, after an edit:
await api.storage.instance.set('todos', todos);
```

## Members

Both scopes have the same members:

| Member            | What it does                                                                    |
| ----------------- | ------------------------------------------------------------------------------- |
| `get(key)`        | Promise of the value saved under `key`, or `undefined` when there is none.      |
| `set(key, value)` | Saves `value` under `key`, replacing the previous one. Values must fit in JSON. |
| `delete(key)`     | Removes `key`. Resolves `true` if it had a value.                               |

In TypeScript, `get` takes the type of the value: `api.storage.instance.get<string[]>('todos')`.

## Where it lives

Every widget's data lives in one file, `widget-data.json` in EdgeStation's data folder (`%APPDATA%\com.edgestation.dashboard` on Windows, `~/.local/share/com.edgestation.dashboard` on Linux), rewritten whole on each change (at most every 100 ms). Keep it to small data: settings, lists, ids, a cache of a few kilobytes.

Nothing is deleted for you: data stays when the placement, its page or the widget is removed. Use `delete(key)` for what you no longer need.

## Sharing between copies

A value in `api.storage.widget` is shared by every copy of the widget on the Edge, but nothing tells the others when one copy changes it. To keep them in step, read the value again right before changing it, so one copy doesn't overwrite what another saved, and poll to pick up the other copies' changes:

```js
let entries = (await api.storage.widget.get('entries')) ?? [];

api.every(2000, async () => {
	entries = (await api.storage.widget.get('entries')) ?? [];
	draw(entries);
});
```

The [`logs-timeline` widget](https://github.com/chainlist/edgestation/tree/main/widgets/logs-timeline) keeps a timeline that every copy shows and edits. The [`todo` widget](https://github.com/chainlist/edgestation/tree/main/widgets/todo) keeps one list for all its copies.
