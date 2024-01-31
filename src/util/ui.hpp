#pragma once

#include <defs.hpp>
#include <functional>

namespace util::ui {
    constexpr cocos2d::ccColor4B BG_COLOR_BROWN = {191, 114, 62, 255};
    constexpr cocos2d::ccColor4B BG_COLOR_TRANSPARENT = {0, 0, 0, 180};

    void switchToScene(cocos2d::CCLayer* layer);
    void prepareLayer(cocos2d::CCLayer* layer);
    void addBackground(cocos2d::CCNode* layer);
    void addBackButton(cocos2d::CCMenu* menu, std::function<void()> callback);
    void navigateBack();
}
