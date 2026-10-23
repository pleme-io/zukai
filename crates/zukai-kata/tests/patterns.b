def default_two_region()
  kata_multi_region({title: "T", subtitle: "S"})
end

def parameterized_region()
  kata_multi_region(
    {
      title: "Parameterized",
      primary: {
        id: "primary",
        label: "Primary",
        role: :master,
        services: [{id: "svc-a", label: "Service A", role: :master}]
      },
      replicas: [
        {
          id: "r1",
          label: "Replica 1",
          role: :read,
          services: [{id: "svc-a-r", label: "A (ro)", role: :read}]
        }
      ],
      gateway: {label: "Client network"}
    }
  )
end

def welcome_hero()
  kata_hero({title: "W", subtitle: "to zukai"})
end

def all_clear()
  kata_closing({label: "All clear", role: :success})
end

def four_pillars()
  kata_pillars(
    {
      origin_x: 40,
      origin_y: 200,
      width: 880,
      pillar_height: 120,
      gap: 20,
      pillars: [
        {id: "p1", label: "One", role: :master},
        {id: "p2", label: "Two", role: :read},
        {id: "p3", label: "Three", role: :accent},
        {id: "p4", label: "Four", role: :neutral}
      ]
    }
  )
end

def three_services()
  kata_service_tier(
    {
      region_id: "primary",
      region_origin_x: 40,
      region_origin_y: 200,
      region_inner_top: 62,
      services: [
        {id: "a", label: "A", role: :master},
        {id: "b", label: "B", role: :master},
        {id: "c", label: "C", role: :master}
      ]
    }
  )
end

def three_layers()
  kata_architecture_layers(
    {
      origin_x: 60,
      origin_y: 120,
      width: 840,
      layer_height: 80,
      gap: 24,
      layers: [
        {id: "p", label: "Presentation", role: :accent},
        {id: "d", label: "Domain", role: :master},
        {id: "i", label: "Infra", role: :neutral}
      ]
    }
  )
end

def outbound_gateway()
  kata_customer_gateway({label: "Client network — outbound only"})
end

def sealed_exchange()
  kata_zero_knowledge(
    {
      client_id: "c",
      client_label: "Client (holds keys)",
      service_id: "s",
      service_label: "Service (ciphertext)",
      client_x: 60,
      client_y: 240,
      service_x: 720,
      service_y: 240,
      payload_label: "ciphertext only"
    }
  )
end

def failover_pair()
  kata_ha_failover(
    {
      primary: {id: "p", label: "Primary"},
      standby: {id: "s", label: "Standby"},
      primary_x: 100,
      primary_y: 200,
      standby_x: 660,
      standby_y: 200,
      failover_label: "failover",
      recovery_label: "recover"
    }
  )
end

def three_replicas()
  kata_read_replica_pair(
    {
      master_id: "m",
      master_label: "Master",
      master_x: 380,
      master_y: 120,
      replicas: [
        {id: "r1", label: "Replica 1"},
        {id: "r2", label: "Replica 2"},
        {id: "r3", label: "Replica 3"}
      ],
      replicas_y: 280,
      canvas_width: 960
    }
  )
end

def three_backends()
  kata_api_fanout(
    {
      api_id: "api",
      api_label: "API",
      api_x: 410,
      api_y: 200,
      backends: [
        {id: "b1", label: "Backend 1", role: :master},
        {id: "b2", label: "Backend 2", role: :master},
        {id: "b3", label: "Backend 3", role: :master}
      ],
      backends_y: 350,
      canvas_width: 960
    }
  )
end

def status_decision()
  kata_decision_branch(
    {
      decision_id: "d",
      decision_label: "Status?",
      decision_x: 380,
      decision_y: 120,
      branches: [
        {id: "ok", label: "Continue", role: :success, branch_label: "ok"},
        {id: "err", label: "Abort", role: :danger, branch_label: "error"}
      ],
      branches_y: 280,
      canvas_width: 960
    }
  )
end

def three_branch_tree()
  kata_tree_hierarchy(
    {
      root: {id: "root", label: "Root", role: :accent},
      root_x: 380,
      root_y: 120,
      branches: [
        {id: "b1", label: "B1", role: :master},
        {id: "b2", label: "B2", role: :master},
        {id: "b3", label: "B3", role: :master}
      ],
      branches_y: 240,
      canvas_width: 960
    }
  )
end

def dns_to_service()
  kata_routing_detail(
    {
      origin_x: 40,
      origin_y: 240,
      stage_width: 160,
      stage_height: 60,
      gap: 40,
      stages: [
        {id: "dns", label: "DNS", role: :neutral, arrow_label: "CNAME"},
        {id: "lb", label: "LB", role: :accent, arrow_label: "HTTP"},
        {id: "svc", label: "Service", role: :master}
      ]
    }
  )
end

def hot_warm_cold()
  kata_data_tier(
    {
      origin_x: 200,
      origin_y: 120,
      width: 560,
      tier_height: 80,
      gap: 30,
      tiers: [
        {id: "hot", label: "Hot", role: :accent, arrow_label: "age out"},
        {id: "warm", label: "Warm", role: :read, arrow_label: "archive"},
        {id: "cold", label: "Cold", role: :neutral}
      ]
    }
  )
end

def cutover_timeline()
  kata_event_timeline(
    {
      origin_x: 60,
      origin_y: 320,
      width: 840,
      events: [
        {id: "t-30", label: "T-30 · prep", role: :neutral},
        {id: "t-0", label: "T+0 · cutover", role: :accent},
        {id: "t+30", label: "T+30 · verify", role: :success}
      ]
    }
  )
end

def request_response()
  kata_interaction_lane(
    {
      left_id: "c",
      left_label: "Client",
      right_id: "s",
      right_label: "Server",
      origin_x: 60,
      origin_y: 100,
      lane_width: 700,
      lifeline_length: 280,
      messages: [
        {label: "request", from_left_to_right: true, role: :accent},
        {label: "process", from_left_to_right: false, role: :neutral},
        {label: "response", from_left_to_right: false, role: :success}
      ]
    }
  )
end

def ids_of(forms)
  map((f) -> get(f, :id), forms)
end

def misspelled_hero()
  kata_hero({title: "W", subtitel: "typo"})
end

def unknown_role()
  kata_closing({label: "x", role: :purple})
end
