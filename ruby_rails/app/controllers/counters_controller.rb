class CountersController < ApplicationController
  def show_count
    count = PresenceCounter.where(name: 'group_sittings').order(updated_at: :desc).limit(1).pluck(:count).first
    render plain: count.to_s
  end
end
