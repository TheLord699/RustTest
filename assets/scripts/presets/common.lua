-- common.lua - Shared helper functions
CommonAI = {}

function CommonAI.distance(x1, y1, x2, y2)
    local dx = x2 - x1
    local dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
end

function CommonAI.move_toward(current_x, current_y, target_x, target_y, speed)
    local dx = target_x - current_x
    local dy = target_y - current_y
    local dist = CommonAI.distance(current_x, current_y, target_x, target_y)
    
    if dist > 0 then
        return (dx / dist) * speed, (dy / dist) * speed
    end
    return 0, 0
end

return CommonAI