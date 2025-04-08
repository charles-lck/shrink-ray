const eleStopPropagation = (eleId: string, callback?: () => void) => {
  const ele = document.getElementById(eleId);
  ele?.addEventListener('mousedown', (e) => {
    e.stopPropagation();
    if (callback) {
      callback();
    }
  });
}
const eleClick = (eleId: string, callback?: () => void) => {
  const ele = document.getElementById(eleId);
  ele?.addEventListener('click', () => {
    if (callback) {
      callback();
    }
  });
}



export {
  eleStopPropagation,
  eleClick
}