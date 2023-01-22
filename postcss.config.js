import tailwindcss from 'tailwindcss'
import form from '@tailwindcss/forms'
import tailwindConfig from './tailwind.config.js'
import autoprefixer from 'autoprefixer'

export default {
  plugins:[tailwindcss(tailwindConfig), form, autoprefixer]
}