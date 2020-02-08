local key = KEYS[1];
local amount = ARGV[1];

local double = redis.call('HGET', key, 'double');
if ( tonumber(amount) <= tonumber(double) ) then 
    redis.call('HINCRBY', key, 'left', -1 * amount);
    redis.call('HINCRBY', key, 'right', -1 * amount);
    redis.call('HINCRBY', key, 'double', -1 * amount);
    return amount;
else
    return 0;
end
