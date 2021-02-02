module.exports = {
  future: {},
  purge: {
	enabled: true,
    content: ['../src/lib.rs','../src/modules/search_bar.rs','../src/modules/side_bar.rs', '../src/modules/pop_up.rs'],
  },
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
  darkMode : 'media',
}
