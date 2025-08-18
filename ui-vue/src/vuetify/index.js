// Vuetify
import "vuetify/styles";

import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

import { aliases, mdi } from "vuetify/iconsets/mdi-svg";
import colors from "vuetify/util/colors";

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: "mdi",
    aliases,
    sets: {
      mdi,
    },
  },

  theme: {
    themes: {
      light: {
        dark: false,
        colors: {
          primary: colors.lightGreen.darken1,
          secondary: colors.lightGreen.lighten4,
        },
      },
    },
  },
});

export default vuetify;
