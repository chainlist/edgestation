`api.openUrl` opens a web page in the default browser of the machine, outside EdgeStation. It needs `"permissions": ["links"]` in the [manifest](/docs/manifest#permissions).

```ts
openUrl(url: string): Promise<void>
```

```js
link.addEventListener('click', (event) => {
	event.preventDefault();
	api.openUrl(link.href).catch(console.error);
});
```

Only `http` and `https` addresses open. For any other, the promise rejects.

## When to call it

Call it when the user taps a link. The page opens in their browser, away from the Edge, so a call they didn't ask for takes them away from what they were doing. To show a page on the Edge itself, use [`api.webview`](/docs/api/webview) instead.

A plain link in a widget doesn't do this: clicking it would load the page in place of the whole Edge. Catch the click and call `api.openUrl`, as above.
