#pragma once
#include <defs.hpp>
#include <Geode/modify/AppDelegate.hpp>

class $modify(GlobedAppDelegate, AppDelegate) {
    $override
    void applicationDidEnterBackground();

    $override
    void applicationWillEnterForeground();
};
