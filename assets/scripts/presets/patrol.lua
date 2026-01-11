PatrolAI = {}

function PatrolAI.update(enemy, player, dt)
    local dx = player.x - enemy.x
    local dy = player.y - enemy.y
    local distance = math.sqrt(dx * dx + dy * dy)
    
    if distance > 0 then
        local speed = enemy.stats and enemy.stats.speed or 2.0
        enemy.vx = (dx / distance) * speed
        enemy.vy = (dy / distance) * speed
    else
        enemy.vx = 0
        enemy.vy = 0
    end
    
    return enemy.vx, enemy.vy
end

return PatrolAI