request = function()
    left = math.random(0, 10000)
    right = math.random(left, 10000)
    amount = math.random(0, 5000)
    path = "/" .. left .. "/" .. right .. "/" .. amount
    return wrk.format(nil, path)
 end