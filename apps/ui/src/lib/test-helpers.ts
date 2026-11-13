/** Lets the waiting promise callbacks run before a test checks the result. */
export const flush = () => new Promise((resolve) => setTimeout(resolve));
