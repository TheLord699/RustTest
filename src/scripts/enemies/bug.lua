BugAI = {}

function BugAI.update(enemy, player, dt)
    if player.x > enemy.x then
        enemy.vx = 50
    else
        enemy.vx = -50
    end
end

return BugAI
