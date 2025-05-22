import config_lang_serder

a = config_lang_serder.read("../test.toml")
config_lang_serder.write(a, "../test_created.json")

# fail = config_lang_serder.read("../test.html")  # will throw unsupported file extension
pass
