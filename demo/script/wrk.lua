
init = function(args)
    max_amount, entity_count = args[1], args[2]
    if max_amount == nil then max_amount = 100  end
    if entity_count == nil then entity_count = 10000  end
end

request = function()
    left = math.random(0, entity_count)
    right = math.random(left + 1, entity_count)
    if (left + 1 == entity_count) then
        right = left
        left = left - 1
    end

    amount = math.random(0, max_amount)

    path = "/" .. left .. "/" .. right .. "/" .. amount
    return wrk.format(nil, path)
 end