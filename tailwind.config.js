const { borderRadius } = require('tailwindcss/defaultTheme')
module.exports = {
  purge: [],
  theme: {
    extend: { colors: {
      primary: '#5c6ac4',
      secondary: '#ecc94b', 
      // ...
    },
  borderRadius:{
    ...borderRadius,
    fff:"1rem"
  }},
   
  },
  variants: {},
  plugins: [],
}
