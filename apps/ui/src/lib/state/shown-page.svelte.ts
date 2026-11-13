/**
 * The page whose tab is open on the Pages settings screen. It is the last tab picked, or the last
 * page opened in the editor, so the user comes back to it. It stays while the settings window is
 * open.
 */
export const shownPage = $state<{ id?: string }>({});
