# sman changes

## 0.3.0 – 2020-01-11
- Fix parsing page names containing period like `pam_env.conf.5.gz`
- Open found files directly instead of calling `man $sec $page`
- De-duplicate manual pages
- Add logging support for debugging

## 0.2.0 – 2019-11-12
- Update dependencies
- Add lockfile
- Use `man --path` instead of hardcoded `/usr/share/man` for finding pages

## 0.1.0 – 2016-12-13
- Initial release
