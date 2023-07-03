ftd.post_init = function () {
    let DARK_MODE = false;
    let SYSTEM_DARK_MODE = false;
    let FOLLOW_SYSTEM_DARK_MODE = false;
    const DARK_MODE_COOKIE = "fastn-dark-mode";
    const COOKIE_SYSTEM_LIGHT = "system-light";
    const COOKIE_SYSTEM_DARK = "system-dark";
    const COOKIE_DARK_MODE = "dark";
    const COOKIE_LIGHT_MODE = "light";
    const DARK_MODE_CLASS = "dark";


    /*
        ftd.dark-mode behaviour:

        ftd.dark-mode is a boolean, default false, it tells the UI to show
        the UI in dark or light mode. Themes should use this variable to decide
        which mode to show in UI.

        ftd.follow-system-dark-mode, boolean, default true, keeps track if
        we are reading the value of `dark-mode` from system preference, or user
        has overridden the system preference.

        These two variables must not be set by ftd code directly, but they must
        use `$on-click$: message-host enable-dark-mode`, to ignore system
        preference and use dark mode. `$on-click$: message-host
        disable-dark-mode` to ignore system preference and use light mode and
        `$on-click$: message-host follow-system-dark-mode` to ignore user
        preference and start following system preference.

        we use a cookie: `ftd-dark-mode` to store the preference. The cookie can
        have three values:

           cookie missing /          user wants us to honour system preference
               system-light          and currently its light.

           system-dark               follow system and currently its dark.

           light:                    user prefers light

           dark:                     user prefers light

        We use cookie instead of localstorage so in future `fpm-repo` can see
        users preferences up front and renders the HTML on service wide
        following user's preference.

     */
    window.enable_dark_mode = function () {
        // TODO: coalesce the two set_bool-s into one so there is only one DOM
        //       update
        DARK_MODE = true;
        FOLLOW_SYSTEM_DARK_MODE = false;
        SYSTEM_DARK_MODE = system_dark_mode();
        document.body.classList.add(DARK_MODE_CLASS);
        set_cookie(DARK_MODE_COOKIE, COOKIE_DARK_MODE);
    };
    window.enable_light_mode = function () {
        // TODO: coalesce the two set_bool-s into one so there is only one DOM
        //       update
        DARK_MODE = false;
        FOLLOW_SYSTEM_DARK_MODE = false;
        SYSTEM_DARK_MODE = system_dark_mode();
        if (document.body.classList.contains(DARK_MODE_CLASS)) {
            document.body.classList.remove(DARK_MODE_CLASS);
        }
        set_cookie(DARK_MODE_COOKIE, COOKIE_LIGHT_MODE);
    };
    window.enable_system_mode = function () {
        // TODO: coalesce the two set_bool-s into one so there is only one DOM
        //       update
        FOLLOW_SYSTEM_DARK_MODE = true;
        SYSTEM_DARK_MODE = system_dark_mode();
        if (SYSTEM_DARK_MODE) {
            DARK_MODE = true;
            document.body.classList.add(DARK_MODE_CLASS);
            set_cookie(DARK_MODE_COOKIE, COOKIE_SYSTEM_DARK);
        }
        else {
            DARK_MODE = false;
            if (document.body.classList.contains(DARK_MODE_CLASS)) {
                document.body.classList.remove(DARK_MODE_CLASS);
            }
            set_cookie(DARK_MODE_COOKIE, COOKIE_SYSTEM_LIGHT);
        }
    };
    function set_cookie(name, value) {
        document.cookie = name + "=" + value + "; path=/";
    }
    function system_dark_mode() {
        return !!(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
    }
    function initialise_dark_mode() {
        update_dark_mode();
        start_watching_dark_mode_system_preference();
    }
    function get_cookie(name, def) {
        // source: https://stackoverflow.com/questions/5639346/
        let regex = document.cookie.match('(^|;)\\s*' + name + '\\s*=\\s*([^;]+)');
        return regex !== null ? regex.pop() : def;
    }
    function update_dark_mode() {
        let current_dark_mode_cookie = get_cookie(DARK_MODE_COOKIE, COOKIE_SYSTEM_LIGHT);
        switch (current_dark_mode_cookie) {
            case COOKIE_SYSTEM_LIGHT:
            case COOKIE_SYSTEM_DARK:
                window.enable_system_mode();
                break;
            case COOKIE_LIGHT_MODE:
                window.enable_light_mode();
                break;
            case COOKIE_DARK_MODE:
                window.enable_dark_mode();
                break;
            default:
                console_log("cookie value is wrong", current_dark_mode_cookie);
                window.enable_system_mode();
        }
    }
    function start_watching_dark_mode_system_preference() {
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener("change", update_dark_mode);
    }
    initialise_dark_mode();
}