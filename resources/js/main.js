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
  sssionStorage.setItem("metasyn-light", document.body.classList.contains('light'))
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
