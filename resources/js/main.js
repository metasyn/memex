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
  getById('sun').classList.toggle('hidden');
  getById('moon').classList.toggle('hidden');
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
