function bug_update()
    entity:move(2, 0)

    if entity:x() > 600 then
        entity:move(-4, 0)
    end
end
