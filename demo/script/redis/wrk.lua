
init = function(args)
    evalsha = args[1]
    entity_count = args[2]
end

request = function()
    left = math.random(0, entity_count)
    right = math.random(left + 1, entity_count)
    if (left + 1 == entity_count) then
        right = left
        left = left - 1
    end
    amount = math.random(0, 5000)

    path = "/evalsha/" .. evalsha .. "/1/" .. (left * 100000 + right) .. "/" .. amount
    return wrk.format(nil, path)
end