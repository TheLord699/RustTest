-- assets/scripts/enemies/bug.lua
BugAI = {}

function BugAI.update(enemy, player, dt)
    local dx = player.x - enemy.x
    local dy = player.y - enemy.y
    local distance = math.sqrt(dx * dx + dy * dy)
    
    local speed = enemy.stats and enemy.stats.speed or 3.0
    local aggro_range = enemy.stats and enemy.stats.aggro_range or 300
    local vx, vy = 0, 0
    
    if distance > 0 and distance < aggro_range then
        -- Normalize direction vector and apply speed
        vx = (dx / distance) * speed
        vy = (dy / distance) * speed
    end
    
    -- Set velocity in enemy table (this is what Rust reads)
    enemy.vx = vx
    enemy.vy = vy
    
    -- You can still return them if needed
    return vx, vy
end

return BugAI