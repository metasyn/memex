/*
 * utils
 *
 */

const getById = (id) => document.getElementById(id);

const addListener = (el, event, fn) => {
  el.addEventListener(event, fn);
};

/*
 * actions
 *
 */

const toggleMode = () => {
  document.body.classList.toggle('light');
  getById('light-mode').classList.toggle('dark');
};

/*
 * setup
 *
 */

const addListeners = () => {
  addListener(
    getById('light-mode'),
    'click',
    toggleMode,
  );
};

addListeners();
