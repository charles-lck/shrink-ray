function generateRandomId() {
  return crypto.getRandomValues(new Uint32Array(2)).join('').slice(0, 8);
}

export { generateRandomId };