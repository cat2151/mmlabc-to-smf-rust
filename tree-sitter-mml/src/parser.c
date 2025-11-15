#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 11
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 13
#define ALIAS_COUNT 0
#define TOKEN_COUNT 7
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_SQUOTE = 1,
  sym_note = 2,
  sym_octave_up = 3,
  sym_octave_down = 4,
  anon_sym_o = 5,
  aux_sym_octave_set_token1 = 6,
  sym_source_file = 7,
  sym__item = 8,
  sym_chord = 9,
  sym_octave_set = 10,
  aux_sym_source_file_repeat1 = 11,
  aux_sym_chord_repeat1 = 12,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_SQUOTE] = "'",
  [sym_note] = "note",
  [sym_octave_up] = "octave_up",
  [sym_octave_down] = "octave_down",
  [anon_sym_o] = "o",
  [aux_sym_octave_set_token1] = "octave_set_token1",
  [sym_source_file] = "source_file",
  [sym__item] = "_item",
  [sym_chord] = "chord",
  [sym_octave_set] = "octave_set",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_chord_repeat1] = "chord_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_note] = sym_note,
  [sym_octave_up] = sym_octave_up,
  [sym_octave_down] = sym_octave_down,
  [anon_sym_o] = anon_sym_o,
  [aux_sym_octave_set_token1] = aux_sym_octave_set_token1,
  [sym_source_file] = sym_source_file,
  [sym__item] = sym__item,
  [sym_chord] = sym_chord,
  [sym_octave_set] = sym_octave_set,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_chord_repeat1] = aux_sym_chord_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_note] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_up] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_down] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_o] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_octave_set_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__item] = {
    .visible = false,
    .named = true,
  },
  [sym_chord] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_set] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_chord_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      if (lookahead == '\'') ADVANCE(2);
      if (lookahead == '<') ADVANCE(4);
      if (lookahead == '>') ADVANCE(5);
      if (lookahead == 'o') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      if (('A' <= lookahead && lookahead <= 'G') ||
          ('a' <= lookahead && lookahead <= 'g')) ADVANCE(3);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_note);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_octave_up);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_octave_down);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_o);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(aux_sym_octave_set_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_note] = ACTIONS(1),
    [sym_octave_up] = ACTIONS(1),
    [sym_octave_down] = ACTIONS(1),
    [anon_sym_o] = ACTIONS(1),
    [aux_sym_octave_set_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(10),
    [sym__item] = STATE(2),
    [sym_chord] = STATE(2),
    [sym_octave_set] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [sym_octave_up] = ACTIONS(7),
    [sym_octave_down] = ACTIONS(7),
    [anon_sym_o] = ACTIONS(9),
  },
  [2] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_octave_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(11),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(13),
    [sym_octave_up] = ACTIONS(13),
    [sym_octave_down] = ACTIONS(13),
    [anon_sym_o] = ACTIONS(9),
  },
  [3] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_octave_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [anon_sym_SQUOTE] = ACTIONS(17),
    [sym_note] = ACTIONS(20),
    [sym_octave_up] = ACTIONS(20),
    [sym_octave_down] = ACTIONS(20),
    [anon_sym_o] = ACTIONS(23),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(26), 6,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [9] = 1,
    ACTIONS(28), 6,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [18] = 3,
    ACTIONS(30), 1,
      anon_sym_SQUOTE,
    ACTIONS(32), 1,
      sym_note,
    STATE(7), 1,
      aux_sym_chord_repeat1,
  [28] = 3,
    ACTIONS(34), 1,
      anon_sym_SQUOTE,
    ACTIONS(36), 1,
      sym_note,
    STATE(7), 1,
      aux_sym_chord_repeat1,
  [38] = 2,
    ACTIONS(39), 1,
      sym_note,
    STATE(6), 1,
      aux_sym_chord_repeat1,
  [45] = 1,
    ACTIONS(41), 1,
      aux_sym_octave_set_token1,
  [49] = 1,
    ACTIONS(43), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 9,
  [SMALL_STATE(6)] = 18,
  [SMALL_STATE(7)] = 28,
  [SMALL_STATE(8)] = 38,
  [SMALL_STATE(9)] = 45,
  [SMALL_STATE(10)] = 49,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [17] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(8),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [23] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_octave_set, 2),
  [28] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chord, 3),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2),
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2), SHIFT_REPEAT(7),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [43] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_mml(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
